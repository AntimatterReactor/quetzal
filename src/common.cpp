#include "include/common.hpp"

#include <iostream>

namespace qtz
{
	bool BasicIndex::next_valid() const noexcept {return i+1 < len;}
	bool BasicIndex::prev_valid() const noexcept {return i-1 < len;}
	bool isidentchar(char __c) noexcept
	{
		return ((__c >= 'A' && __c <= 'Z') ||
			(__c >= 'a' && __c <= 'z') ||
			(__c >= '0' && __c <= '9') || __c == '_');
	}

	std::string readFile(std::string fileName)
	{
		std::ifstream file(fileName);
		if (!file.is_open())
		{
			std::cerr << "Error: Could not open file " << fileName << std::endl;
			return "";
		}
		std::string contents;
		file.seekg(0, std::ios::end);
		contents.resize(file.tellg());
		file.seekg(0, std::ios::beg);
		file.read(&contents[0], contents.size());
		file.close();
		return contents;
	}
	void writeFile(std::string fileName, std::string contents)
	{
		std::ofstream file(fileName);
		if (!file.is_open())
		{
			std::cerr << "Error: Could not open file " << fileName << std::endl;
			return;
		}
		file << contents;
		file.close();
	}
}
