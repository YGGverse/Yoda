#include "label.hpp"

using namespace app::browser::main::tab;

Label::Label(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init actions
    action__tab_close = ACTION__TAB_CLOSE;

    // Setup label controller
    auto const EVENT__GESTURE_CLICK = Gtk::GestureClick::create();

        /* @TODO remove as default
        controller->set_button(
            GDK_BUTTON_PRIMARY
        );*/

        EVENT__GESTURE_CLICK->signal_pressed().connect(
            [this](int n, double x, double y)
            {
                if (n == 2) // double click
                {
                    action__tab_close->activate();
                }
            }
        );

        add_controller(
            EVENT__GESTURE_CLICK
        );

    // Init widget
    /* not in use, overwritten by page init @TODO
    set_text(
        _("New page")
    );*/
}

// Actions
int Label::session_restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_label__session`
                        WHERE `app_browser_main_tab__session__id` = %d
                        ORDER BY `id` DESC LIMIT 1
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID
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
            set_text(
                reinterpret_cast<const char*>(
                    sqlite3_column_text(
                        statement,
                        DB::SESSION::TEXT
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

int Label::session_save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    // Delegate save action to child components (on available)

    // Save label session
    return DB::SESSION::add(
        db,
        APP_BROWSER_MAIN_TAB__SESSION__ID,
        get_text()
    );
}

// Database model
int Label::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_label__session`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab__session__id` INTEGER NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `text` VARCHAR (1024) NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Label::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_label__session`
                        WHERE `app_browser_main_tab__session__id` = %d
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID
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
                db,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_label__session` WHERE `id` = %d
                    )SQL",
                    sqlite3_column_int64(
                        statement,
                        DB::SESSION::ID
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

sqlite3_int64 Label::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
    const Glib::ustring & TEXT
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_label__session` (
                    `app_browser_main_tab__session__id`,
                    `text`
                ) VALUES (
                    '%d',
                    '%s'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID,
            TEXT
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        db
    );
}