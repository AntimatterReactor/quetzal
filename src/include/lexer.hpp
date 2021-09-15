#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string>
#include <vector>

namespace qtz
{
	enum class token_types
	{
		ARROW,

		RCURL,
		LCURL, 

		FUNC,
		VAR,
		CONST,
		TYPE,
		PRIM,

		STRLIT,
		NUMBERS,
		IDENT,

		EOS,
		NONE,
	};

	struct token
	{
		token_types tt;
		std::string val;
	};
	std::vector<token> lexer(std::string);
}

#endif
