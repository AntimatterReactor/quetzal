#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <cstdint>
#include <string>
#include <vector>

#include "common.hpp"

namespace qtz
{
	enum class TokenTypes
	{
		LPAREN, RPAREN,
		LBRACE, RBRACE,
		LBRACKET, RBRACKET,
		SEMICOLON, COMMA, DOT, COLON,
		PLUS, MINUS, MUL, DIV, MOD, ASSIGN,
		EQ, NEQ, LT, GT, LTE, GTE,
		AND, OR, NOT,
		IF, ELSE, WHILE, FOR,
		FUNC, VAR, CONST, RETURN,
		TRUE, FALSE,
		STRLIT, NUMLIT, NUMMOD,
		IDENT,
		EOF_TOKEN, NONE
	};

	struct Token
	{
		TokenTypes tt;
		std::string val;

		void clear();
	};
	
	class Lexer : protected virtual IndexItem<std::string, char>
	{
	public:
		Lexer(std::string code_string) noexcept
			: IndexItem(code_string, code_string.length()) {}
		~Lexer() {}
	public:
		std::vector<Token> tokens;
		std::uint8_t escape(char) const noexcept;
		Lexer tokenify();
	};
}

#endif
