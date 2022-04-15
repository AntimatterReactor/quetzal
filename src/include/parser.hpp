#ifndef __PARSER_HPP__
#define __PARSER_HPP__

#include "common.hpp"
#include "lexer.hpp"

#include <string>
#include <vector>

namespace qtz
{
class Parser
{
      public:
	Parser(Lexer &lexer);
	~Parser();

	void parse();

      private:
	Lexer &lexer;
	Token currentToken;
	Token previousToken;

	void parseProgram();
	void parseStatement();
	void parseExpression();
	void parseAssignment();
	void parseVariable();
	void parseFunction();
	void parseFunctionCall();
	void parseFunctionArguments();
	void parseIf();
	void parseElse();
	void parseWhile();
	void parseFor();
	void parseReturn();
	void parseBreak();
	void parseContinue();
	void parseBlock();
};
} // namespace qtz

#endif
