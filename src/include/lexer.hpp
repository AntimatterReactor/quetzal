#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string>

namespace qtz
{
	enum class token_types;
	struct token;
	struct token lexer(std::string);
}

#endif
