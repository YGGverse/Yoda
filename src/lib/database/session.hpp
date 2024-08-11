#ifndef LIB_DATABASE_SESSION_H
#define LIB_DATABASE_SESSION_H

#include <sqlite3.h>

namespace lib::database
{
    class Session
    {
        private:

            int status;

            char * error;

            sqlite3 * connection;

        public:

            Session(
                sqlite3 * connection
            );
    };
}

#endif // LIB_DATABASE_SESSION_H