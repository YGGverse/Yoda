#include "database.hpp"
#include "database/session.hpp"

using namespace lib;

Database::Database(
    const char * filename
) {
    status = sqlite3_open(
        filename,
        &connection
    );

    session = new database::Session(
        connection
    );
}