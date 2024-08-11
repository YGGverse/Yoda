#ifndef LIB_DATABASE_H
#define LIB_DATABASE_H

#include <sqlite3.h>

namespace lib
{
    class Database
    {
        private:

            int status;

            char * error;

            sqlite3 * connection;

        public:

            Database(
                const char * filename
            );
    };
}

#endif // LIB_DATABASE_H