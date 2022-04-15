#ifndef __ERROR_HPP__
#define __ERROR_HPP__

#include <exception>
#include <string>

namespace qtz {
struct Location {
	int line;
	int column;
};

class quetzal_error : public std::exception {
	private:
	std::string msg_;
	Location loc_;

	public:
	quetzal_error(const std::string &message, const Location &location)
	    : msg_(message), loc_(location)
	{
	}

	quetzal_error(const char *message, const Location &location)
	    : msg_(message), loc_(location)
	{
	}

	quetzal_error(const quetzal_error &other) noexcept
	    : msg_(other.msg_), loc_(other.loc_)
	{
	}

	virtual ~quetzal_error() noexcept {}

	virtual const char *what() const noexcept { return msg_.c_str(); }

	const Location &where() const noexcept { return loc_; }
};
} // namespace qtz

#endif
