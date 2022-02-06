#ifndef __ERROR_HPP__
#define __ERROR_HPP__

#include <cstdio>
#include <string>

namespace qtz
{
	struct Location
	{
		std::string file;
		int line;
		int column;
	};

	class BasicError
	{
	public:
		BasicError(const std::string& message, const Location where)
			: e_message(message), e_where(where) {}
		~BasicError() {}

		const std::string& getMessage() const;
	protected:
		std::string e_message;
		Location e_where;
	};
}

#endif
