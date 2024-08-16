#ifndef LIB_DATABASE_HPP
#define LIB_DATABASE_HPP

#include <sqlite3.h>

namespace lib
{
    namespace database
    {
        class Session;
    }

    class Database
    {
        int status;

        char * error;

        sqlite3 * connection;

        public:

            database::Session * session;

            Database(
                const char * filename
            );
    };
}

#endif // LIB_DATABASE_HPP