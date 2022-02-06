#ifndef __PARSER_HPP__
#define __PARSER_HPP__

#include "common.hpp"
#include "lexer.hpp"
#include <string>
#include <unordered_set>

namespace qtz
{
	struct TreeNode
	{
		int type;
		Token value;
		TreeNode *parentNode;
		TreeNode *siblingNode;
		TreeNode *childNode;
	};

	class ParseTree : virtual public std::unordered_set<TreeNode*>
	{
	public:
		ParseTree(TreeNode *root) : unordered_set() {}
		~ParseTree() {}
	};

	class Parser
	{

	};
}

#endif

