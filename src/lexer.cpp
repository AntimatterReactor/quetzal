#include "include/lexer.hpp"
#include "include/common.hpp"

#include <bitset>
#include <cctype>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

namespace qtz
{
static const std::unordered_map<std::string, TTypes> Keywords({
    {"fn", TTypes::FUNC},
    {"var", TTypes::VAR},
    {"const", TTypes::CONST},
    {"if", TTypes::IF},
    {"else", TTypes::ELSE},
    {"while", TTypes::WHILE},
    {"for", TTypes::FOR},
    {"return", TTypes::RETURN},
    {"true", TTypes::TRUE},
    {"false", TTypes::FALSE},
});

static const std::unordered_map<std::string, TTypes> Operators({
    {"+", TTypes::PLUS},   {"-", TTypes::MINUS},     {"*", TTypes::MUL},
    {"/", TTypes::DIV},	   {"%", TTypes::MOD},	     {"=", TTypes::ASSIGN},
    {"==", TTypes::EQ},	   {"!=", TTypes::NEQ},	     {"<", TTypes::LT},
    {"<=", TTypes::LTE},   {">", TTypes::GT},	     {">=", TTypes::GTE},
    {"&&", TTypes::AND},   {"||", TTypes::OR},	     {"!", TTypes::NOT},
    {"(", TTypes::LPAREN}, {")", TTypes::RPAREN},    {"{", TTypes::LBRACE},
    {"}", TTypes::RBRACE}, {"[", TTypes::LBRACKET},  {"]", TTypes::RBRACKET},
    {",", TTypes::COMMA},  {";", TTypes::SEMICOLON}, {".", TTypes::DOT},
    {":", TTypes::COLON},
});

void Token::clear() noexcept
{
	this->tt = TTypes::NONE;
	this->val.clear();
}

void Token::push(const char c) noexcept { this->val.push_back(c); }

void Token::push(const TTypes tt, const char c) noexcept
{
	this->tt = tt;
	this->val.push_back(c);
}

// Generic escape function
constexpr char Lexer::escape(const char c) noexcept
{
	switch (c)
	{
	case 'a': return '\a';
	case 'b': return '\b';
	case 'f': return '\f';
	case 'n': return '\n';
	case 'r': return '\r';
	case 't': return '\t';
	case 'v': return '\v';
	case '\'':
	case '\"':
	case '\\':
	default: return c;
	}
}

Lexer Lexer::tokenify()
{
	Token current;
	std::bitset<4> flag(0);

	enum
	{
		FSTR = 0,
		FESC = 1,
		FIDENT = 2,
		FOP = 3,
	};

	for (; this->i < this->len; this->i++)
	{
		if (access_char() == '\"' && !flag[FESC])
		{
			// Handle String Literals
			current.tt = TTypes::STRLIT;
			flag[FSTR].flip();
			if (!flag[FSTR])
			{
				this->tokens.push_back(current);
				current.clear();
			}
		}
		else if (flag[FSTR])
		{
			// Handle Escape Sequences
			if (flag[FESC])
			{
				// Generic handling for escape sequences
				current.push(escape(access_char()));
				flag[FESC] = false;
			}
			else if (access_char() == '\\') flag[FESC] = true;
			else current.push(access_char());
		}

		else if (std::isspace(access_char())) continue;

		else if (std::isdigit(access_char()) && !flag[FIDENT])
		{
			// Handle Number Literals
			current.push(TTypes::NUMLIT, access_char());
			if (!std::isdigit(access_char<1>()))
			{
				this->tokens.push_back(current);
				if (std::isalpha(access_char<1>()))
				{
					current.tt = TTypes::NUMMOD;
					current.val = access_char<1>();
					this->tokens.push_back(current);
					this->i++;
				}
				current.clear();
			}
		}
		else if (flag[FIDENT] || isidentchar(access_char()))
		{
			// Handle Identifiers
			current.push(TTypes::IDENT, access_char());
			flag[FIDENT] = true;
			if (!isidentchar(access_char<1>()) &&
			    !std::isdigit(access_char<1>()))
			{
				if (Keywords.count(current.val))
					current.tt = Keywords.at(current.val);
				this->tokens.push_back(current);
				current.clear();
				flag[FIDENT] = false;
			}
		}
		else if (std::ispunct(access_char()))
		{
			current.push(access_char());
			flag[FOP] = true;
			if (!std::ispunct(access_char<1>()))
			{
				for (int i = 0;; i++)
				{
					if (current.val.empty())
					{
						// Add exception later
						current.tt = TTypes::NONE;
						this->i += i;
						break;
					}
					else if (Operators.count(current.val)) {
						current.tt =
						    Operators.at(current.val);
						break;
					}
					else {
						current.val.pop_back();
						this->i--;
					}
				}
				this->tokens.push_back(current);
				current.clear();
				flag[FOP] = false;
			}
		}
		// TODO: Handle Syntax Errors
	}
	return *this;
}
} // namespace qtz
