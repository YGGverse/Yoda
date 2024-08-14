#ifndef LIB_URL_HPP
#define LIB_URL_HPP

#include <regex>
#include <string>

namespace lib
{
    class Url
    {
        public:

            std::string scheme,
                        host,
                        port,
                        path,
                        query;

            Url(
                std::string subject
            );

            std::string to_string();

            ~Url();
    };
}

#endif // LIB_URL_HPP