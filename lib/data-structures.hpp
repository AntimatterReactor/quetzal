#ifndef __DATA_STRUCTURES__
#define __DATA_STRUCTURES__
#include <stddef.h>
#include <vector>

namespace qtz
{
	template<typename V, typename T>
	struct token
	{
		V value;
		T type;
	};

	template<typename T>
	struct node
	{
		node *parent;
		node *sibling;
		union
		{
			node *child;
			T value;
		};
		size_t index;
	};	
	
	template<typename T>
	class parse_tree
	{
	private:
		std::vector<node<T>*> nodes;
	public:
		parse_tree();
		~parse_tree();
	public:
		bool valid_up();
		bool valid_down();
		bool valid_right();
	public:
		void up();
		void right();
		void down();
	public:
		size_t depth();
		size_t width();
		size_t pos_depth();
		size_t pos_width();
	public:
		void push_down(T);
		void push_left(T);
		void push_right(T);
	};

	
}

#endif

