#include "include/lexer.hpp"

#include <string>
#include <vector>
#include <iostream>

namespace qtz
{
	std::vector<token> lexer(std::string input)
	{
		std::vector<token> result;
		token current;
		bool in_string = false;
		bool is_escaped = false;

		for(int i = 0; i < input.length(); ++i)
		{
			if(input[i] == '\"' && !is_escaped)
			{
				in_string = !in_string;
				current.tt = token_types::STRLIT;
			}
			else if(in_string)
			{
				if(is_escaped)
				{
					if(input[i] == '\\' || input[i] == '\'' || input[i] == '\"')
						current.val.push_back(input[i]);
					else if(input[i] == 'n') current.val.push_back('\n');
					else if(input[i] == 'b') current.val.push_back('\b');
					is_escaped = false;
					continue;
				}
				if(input[i] == '\\')
				{
					is_escaped = true;
					continue;
				}
				else current.val.push_back(input[i]);
			}
		}
		std::cout << current.val << '\n';
		return result;
	}
}
