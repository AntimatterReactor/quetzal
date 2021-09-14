#include "include/lexer.hpp"

#include <string>
#include <vector>

enum class qtz::token_types
{
	EOL,
};

struct qtz::token
{
	qtz::token_types tt;
	std::string val;
};

struct qtz::token qtz::lexer(std::string input)
{
	qtz::token r;
	r.tt = qtz::token_types::EOL;
	r.val = input;
	return r;
}
