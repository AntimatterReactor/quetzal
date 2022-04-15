#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <cstdint>
#include <string>
#include <vector>

#include "common.hpp"

namespace qtz {
// clang-format off
enum class TTypes
{
	LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET,

	SEMICOLON, COMMA, DOT, COLON, NA_FN_CALL,

	PLUS, MINUS, MUL, DIV, MOD,

	ASSIGN, APLUS, AMIN, AMUL, ADIV, AMOD,

	EQ, NEQ, LT, GT, LTE, GTE, AND, OR, NOT,

	IF, ELSE, WHILE, FOR, FUNC, VAR, CONST, RETURN, TRUE,
	FALSE,

	STRLIT, NUMLIT, NUMMOD, IDENT, NONE = -1
};
// clang-format on

struct Token {
	TTypes tt;
	std::string val;

	void clear() noexcept;
	void push(const char c) noexcept;
	void push(const TTypes tt, const char c) noexcept;
};

class Lexer {
	public:
	static constexpr char escape(char) noexcept;
	Lexer tokenify();

	std::vector<Token> tokens;

	protected:
	std::size_t i, len;
	std::string code_;

	private:
	template <long Offset = 0> char access_char() const noexcept
	{
		return i + Offset < len ? code_.at(i + Offset) : 0;
	}

	public:
	Lexer(std::string code_string) noexcept
	    : i(0), len(code_string.size()), code_(std::move(code_string))
	{
	}
	~Lexer() {}
};
} // namespace qtz

#endif
