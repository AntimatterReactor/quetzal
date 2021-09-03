#ifndef __DATA_STRUCTURES__
#define __DATA_STRUCTURES__

namespace qtzl 
{
	template<typename T>
	struct Node
	{
		Node *parent;
		Node *sibling;
		union
		{
			Node *child;
			T value;
		};
	};	
	
	template<typename T>
	class ParseTree
	{
	private:
		Node<T>** nodes;
	public:
		ParseTree();
		~ParseTree();
	};
	

}

#endif

