#include "database.hpp"

using namespace lib;

Database::Database(
    const char * filename
) {
    status = sqlite3_open(
        filename,
        &connection
    );
}