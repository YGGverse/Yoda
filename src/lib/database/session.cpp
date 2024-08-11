#include "session.hpp"

using namespace lib::database;

Session::Session(
    sqlite3 * connection
) {
    status = sqlite3_exec(
        connection,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `session`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024)
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}