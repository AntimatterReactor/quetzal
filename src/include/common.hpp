#ifndef __COMMON_HPP__
#define __COMMON_HPP__

#include <string>
#include <fstream>

namespace qtz
{
	struct BasicIndex
	{
		size_t len;
		size_t i;

		BasicIndex(size_t length, size_t index_start=0) : len(length), i(index_start) {}

		bool next_valid() const noexcept;
		bool prev_valid() const noexcept;
	};

	template<class Container, typename Value>
	struct IndexItem : public BasicIndex
	{
		Container indexable;
		IndexItem(Container iterable_item, size_t length, size_t index_start=0)
			: BasicIndex(length, index_start), indexable(iterable_item) {}

		Value prev_char() const noexcept;
		Value curr_char() const noexcept;
		Value next_char() const noexcept;
	};

	bool isidentchar(char) noexcept;
	std::string readFile(std::string);

	// NOTE: IndexItem class-functions is defined here because templates
	template<class Container, typename Value>
	Value IndexItem<Container, Value>::prev_char() const noexcept
	{return this->prev_valid() ? this->indexable[this->i-1] : 0 ;}

	template<class Container, typename Value>
	Value IndexItem<Container, Value>::curr_char() const noexcept
	{return this->indexable[this->i];}

	template<class Container, typename Value>
	Value IndexItem<Container, Value>::next_char() const noexcept
	{return this->next_valid() ? this->indexable[this->i+1] : 0 ;}
}

#endif
