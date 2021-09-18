#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string>
#include <vector>

namespace qtz
{
	enum class token_types
	{
		ARROW=1,

		RCURL,
		LCURL,

		FUNC,
		VAR,
		CONST,
		PRIMTV,

		STRLIT,
		NUMBERS,
		NUMMOD,
		IDENT,
		
		EOS=0,
		NONE=-1,
		UNKNOWN=-2,
	};

	struct token
	{
		token_types tt;
		std::string val;

		void clear();
	};
	std::vector<token> lexer(std::string);
}

#endif
