#ifndef __COMMON_HPP__
#define __COMMON_HPP__

#include <fstream>
#include <string>

namespace qtz {
static constexpr bool isidentchar(char __c) noexcept
{
	return (std::isalpha(__c) || __c == '_');
}

std::string readFile(std::string);
} // namespace qtz

#endif
