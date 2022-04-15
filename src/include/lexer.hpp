#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <cstdint>
#include <string>
#include <vector>

#include "common.hpp"

namespace qtz
{
	enum class TTypes
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
		NONE = -1
	};

	struct Token
	{
		TTypes tt;
		std::string val;

		void clear() noexcept;
		void push(const char c) noexcept;
		void push(const TTypes tt, const char c) noexcept;
	};
	
	class Lexer
	{
	public:
		static constexpr char escape(char) noexcept;
		Lexer tokenify();

	protected:
		std::vector<Token> tokens;
		std::size_t i, len;
		std::string code_;

	private:
		template<long N>
		constexpr bool offset_valid() noexcept { return i+N < len; }

		template<long Offset = 0>
		char access_char() const noexcept
		{ return offset_valid<Offset> ? code_.at(i+Offset) : 0; }

	public:
		Lexer(std::string code_string) noexcept
			: i(0), len(code_string.size()), code_(std::move(code_string)) {}
		~Lexer() {}
	};
}

#endif
