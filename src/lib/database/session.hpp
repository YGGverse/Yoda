#ifndef LIB_DATABASE_SESSION_HPP
#define LIB_DATABASE_SESSION_HPP

#include <sqlite3.h>

namespace lib::database
{
    class Session
    {
        int status;

        char * error;

        sqlite3 * connection;

        public:

            Session(
                sqlite3 * connection
            );
    };
}

#endif // LIB_DATABASE_SESSION_HPP