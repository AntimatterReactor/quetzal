#include "include/lexer.hpp"

#include <string>
#include <vector>
#include <iostream>

#include <cstring>

const char* help_msg = \
"\
Usage: qtz [options] ... [file]\n\
\n\
Options:\n\
	-h, --help	Display this message and exit\n\
	-v, --version	Display current version and exit\n\
";

int front_end(int argc, const char** argv)
{
	if (argc < 2)
	{
		std::cout << help_msg << std::flush;
		return 0;
	}

	if (std::strcmp(argv[1], "--help") == 0 || std::strcmp(argv[1], "-h") == 0)
	{
		std::cout << help_msg << std::flush;
		return 0;
	}
	
	for (auto i : qtz::lexer("fn int main const int12v 12L var _string2 \"lol\"12MLOL"))
	{
		std::cout << i.val << ' ' << static_cast<int>(i.tt) << '\n';
	}
	return 0;
}

int main(int argc, const char** argv)
{
	front_end(argc, argv);
	return 0;
}
