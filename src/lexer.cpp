#include "include/lexer.hpp"

#include <cctype>
#include <string>
#include <vector>
#include <iostream>

namespace qtz
{
	void token::clear()
	{
		this->tt = token_types::NONE;
		this->val.erase(this->val.begin(), this->val.end());
	}

	std::vector<token> lexer(std::string input)
	{
		std::vector<token> result;
		token current;
		bool in_string = false;
		bool is_escaped = false;
		bool is_ident = false;

		for(int i = 0; i < input.length(); ++i)
		{
			if(input[i] == '\"' && !is_escaped)
			{
				current.tt = token_types::STRLIT;
				in_string = !in_string;
				if(!in_string)
				{
					result.push_back(current);
					current.clear();
				}
			}
			else if(in_string)
			{
				if(is_escaped)
				{
					switch(input[i]) 
					{
					case 'n': current.val.push_back('\n'); break;
					case 'b': current.val.push_back('\b'); break;
					case 't': current.val.push_back('\t'); break;
					case 'v': current.val.push_back('\v'); break;
					case '\\':
					case '\'':
					case '\"': current.val.push_back(input[i]); break;
					}
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
			else if(std::isdigit(input[i]) && !is_ident)
			{
				current.tt = token_types::NUMBERS;
				current.val.push_back(input[i]);
				if(i + 1 < input.length() ? !std::isdigit(input[i+1]): true)
				{
					result.push_back(current);
					current.clear();
				}
			}
			else if(std::isalnum(input[i]) || input[i] == '_')
			{
				if(std::isdigit(input[i-1]) && current.tt == token_types::NONE)
				{
					current.tt = token_types::NUMMOD;
					current.val.push_back(input[i]);
					result.push_back(current);
					current.clear();
				}
				else
				{
					current.tt = token_types::IDENT;
					current.val.push_back(input[i]);
					is_ident = true;
					if(i + 1 < input.length() ? !std::isalnum(input[i+1]) && input[i] != '_' : true)
					{
						if(current.val == "fn") current.tt = token_types::FUNC;
						else if(current.val == "var") current.tt = token_types::VAR;
						else if(current.val == "const") current.tt = token_types::CONST;
						result.push_back(current);
						current.clear();
						is_ident = false;
					}
				}
			}
		}
		return result;
	}
}
