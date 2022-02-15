#include "include/lexer.hpp"

#include <string>
#include <vector>
#include <iostream>
#include <cstring>

#define OPTION(f, s, l) !(std::strcmp(f, s) && std::strcmp(f, l))

static inline void print_help() {
	std::cout
		<< "Usage: quetzal [options] files..." << '\n'
		<< "Options:" << '\n'
		<< "\t-h, --help\t\t\tPrint this help message" << std::endl;
}

static int front_end(const int argc, const char** argv) {
	if (argc < 2) {
		print_help();
		return 0;
	}

	if (OPTION(argv[1], "-h", "--help")) {
		print_help();
		return 0;
	}

	std::string ss = "fn int main() {\n\tconst int12v = 12L;\n\tvar _string2 = \"lol\";\n\t12L&&int12v";
	std::cout << ss << '\n';
	qtz::Lexer lexer (ss);
	for (auto i : lexer.tokenify().tokens) {
		std::cout << i.val << ' ' << static_cast<int>(i.tt) << '\n';
	}
	return 0;
}

int main(const int argc, const char** argv)
{
	front_end(argc, argv);
	return 0;
}
