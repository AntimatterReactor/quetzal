#include "include/lexer.hpp"
#include "include/common.hpp"

#include <string>
#include <vector>
#include <iostream>
#include <bitset>
#include <unordered_map>

namespace qtz
{
	static const std::unordered_map<std::string, TokenTypes> Keywords
	(
	 {
		{"fn", TokenTypes::FUNC},
		{"var", TokenTypes::VAR},
		{"const", TokenTypes::CONST},
		{"if", TokenTypes::IF},
		{"else", TokenTypes::ELSE},
		{"while", TokenTypes::WHILE},
		{"for", TokenTypes::FOR},
		{"return", TokenTypes::RETURN},
		{"true", TokenTypes::TRUE},
		{"false", TokenTypes::FALSE},
	 }
	);


	static const std::unordered_map<std::string, TokenTypes> Operators
	(
	 {
		{"+", TokenTypes::PLUS}, {"-", TokenTypes::MINUS},
		{"*", TokenTypes::MUL}, {"/", TokenTypes::DIV},
		{"%", TokenTypes::MOD},
		{"=", TokenTypes::ASSIGN},
		{"==", TokenTypes::EQ}, {"!=", TokenTypes::NEQ},
		{"<", TokenTypes::LT}, {"<=", TokenTypes::LTE},
		{">", TokenTypes::GT}, {">=", TokenTypes::GTE},
		{"&&", TokenTypes::AND}, {"||", TokenTypes::OR}, {"!", TokenTypes::NOT},
		{"(", TokenTypes::LPAREN}, {")", TokenTypes::RPAREN},
		{"{", TokenTypes::LBRACE}, {"}", TokenTypes::RBRACE},
		{"[", TokenTypes::LBRACKET}, {"]", TokenTypes::RBRACKET},
		{",", TokenTypes::COMMA}, {";", TokenTypes::SEMICOLON},
		{".", TokenTypes::DOT}, {":" , TokenTypes::COLON},
	 }
	);

	void Token::clear()
	{
		this->tt = TokenTypes::NONE;
		this->val.erase(this->val.begin(), this->val.end());
	}

	// Generic escape function
	std::uint8_t Lexer::escape(char input_char) const noexcept
	{
		char typescp = '0';
		switch(input_char)
		{
			case 'a': return '\a';
			case 'b': return '\b';
			case 'f': return '\f';
			case 'n': return '\n';
			case 'r': return '\r';
			case 't': return '\t';
			case 'v': return '\v';
			case '\'': // FALLTHROUGH 
			case '\"': // FALLTHROUGH 
			case '\\': return input_char;
			case '0': if (!isdigit(next_char())) return '\0'; else typescp = 'o'; break;
			case 'x': typescp = 'x'; break;
			default:
			// TODO: Add Warning Mechanism Here
				break;
		}

		return input_char;
	}

	Lexer Lexer::tokenify()
	{
		std::vector<Token> result;
		Token current;
		std::bitset<0> flag (0);

		enum Flags
		{
			FSTR   = 0,
			FESC   = 1,
			FIDENT = 2,
		};

		for(; this->i < this->len; this->i++)
		{
			if(curr_char() == '\"' && !flag[FESC]) // Handle String Literals
			{
				current.tt = TokenTypes::STRLIT;
				flag[FSTR].flip();
				if(!flag[FSTR])
				{
					result.push_back(current);
					current.clear();
				}
			}
			else if(flag[FSTR]) // Handle Escape Sequences
			{
				if(flag[FESC])
				{
					// Generic handling for escape sequences
					current.val.push_back(escape(curr_char()));
					flag[FESC] = false;
					continue;
				}
				if(curr_char() == '\\')
				{
					flag[FESC] = true;
					continue;
				}
				else current.val.push_back(curr_char());
			}
			else if(std::isdigit(curr_char())) // Handle Number Literals
			{
				current.tt = TokenTypes::NUMLIT;
				current.val.push_back(curr_char());
				if(!std::isdigit(next_char()))
				{
					result.push_back(current);
					if(std::isalpha(next_char()))
					{
						current.tt = TokenTypes::NUMMOD;
						current.val = next_char();
						result.push_back(current);
						this->i++;
					}
					current.clear();
				}
			}
			else if(isidentchar(curr_char())) // Handle Identifiers
			{
				current.tt = TokenTypes::IDENTIFIER;
				current.val.push_back(curr_char());
				flag[FIDENT] = true;
				if(!isidentchar(next_char()))
				{
					result.push_back(current);
					current.clear();
					flag[FIDENT] = false;
				}
			}
			else
			{

			}
		}
		this->tokens = result;
		return *this;
	}

	Lexer Lexer::categorify()
	{
		
	}
}
