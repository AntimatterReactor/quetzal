#ifndef __INTERPRETER_HPP__
#define __INTERPRETER_HPP__

namespace qtz
{
struct ByteCode
{
	int opcode;
	int arg1;
	int arg2;

	enum OpCodes
	{
		OP_DECL,
		OP_ADD,
		OP_SUB,
	};
};

class Compiler
{
      public:
	Compiler();
	~Compiler();
};

class Interpreter
{
      public:
	Interpreter();
	~Interpreter();
};
} // namespace qtz

#endif
