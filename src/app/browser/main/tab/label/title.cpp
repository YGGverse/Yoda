#include "title.hpp"

using namespace app::browser::main::tab::label;

Title::Title(
    sqlite3 * database
) {
    // Init database
    Database::Session::init(
        this->database = database
    );

    // Init extras
    text = _("New page");

    // Init widget
    set_text(
        text
    );

    set_ellipsize(
        Pango::EllipsizeMode::END
    );

    set_width_chars(
        WIDTH_CHARS
    );

    set_single_line_mode(
        true
    );
}

// Actions
int Title::session_restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
) {
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_label_title__session`
                        WHERE `app_browser_main_tab_label__session__id` = %d
                        ORDER BY `id` DESC LIMIT 1
            )SQL",
            APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
        ).c_str(),
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        // Restore label text from latest database record
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore widget data
            update(
                reinterpret_cast<const char*>(
                    sqlite3_column_text(
                        statement,
                        Database::Session::TEXT
                    )
                )
            );

            // Restore children components here (on available)
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Title::session_save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
) {
    // Create new session record
    const sqlite3_int64 APP_BROWSER_MAIN_TAB_LABEL_TITLE__SESSION__ID = Database::Session::add(
        database,
        APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID,
        text
    );

    // Delegate save action to child components (on available)

    // Return ID
    return APP_BROWSER_MAIN_TAB_LABEL_TITLE__SESSION__ID;
}

void Title::update(
    const Glib::ustring & TEXT
) {
    set_text(
        text = TEXT
    );
}

// Database model
int Title::Database::Session::init(
    sqlite3 * database
) {
    char * error;

    return sqlite3_exec(
        database,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_label_title__session`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab_label__session__id` INTEGER NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `text` VARCHAR(1024) NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Title::Database::Session::clean(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_label_title__session`
                        WHERE `app_browser_main_tab_label__session__id` = %d
            )SQL",
            APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
        ).c_str(),
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                database,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_label_title__session` WHERE `id` = %d
                    )SQL",
                    sqlite3_column_int64(
                        statement,
                        Database::Session::ID
                    )
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                // nothing here.
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Title::Database::Session::add(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID,
    Glib::ustring & TEXT
) {
    char * error; // @TODO

    sqlite3_exec(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_label_title__session` (
                    `app_browser_main_tab_label__session__id`,
                    `text`
                ) VALUES (
                    '%d',
                    '%s'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID,
            TEXT
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        database
    );
}