#include "label.hpp"

using namespace app::browser::main::tab;

Label::Label(
    sqlite3 * database,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE
) {
    // Init database
    Database::Session::init(
        this->database = database
    );

    // Init actions
    action__tab_close = ACTION__TAB_CLOSE;

    // Init extras
    text = _("New page");

    is_pinned = false;

    // Init widget
    set_ellipsize(
        Pango::EllipsizeMode::END
    );

    set_has_tooltip(
        true
    );

    set_single_line_mode(
        true
    );

    set_width_chars(
        WIDTH_CHARS
    );

    // Init primary button controller
    const auto EVENT__BUTTON_PRIMARY = Gtk::GestureClick::create();

        EVENT__BUTTON_PRIMARY->set_button(
            GDK_BUTTON_PRIMARY
        );

        add_controller(
            EVENT__BUTTON_PRIMARY
        );

        // Connect events
        EVENT__BUTTON_PRIMARY->signal_pressed().connect(
            [this](int n, double x, double y)
            {
                if (n == 2) // double click
                {
                    pin(
                        !is_pinned // toggle
                    );
                }
            }
        );

    // Init middle button controller
    const auto EVENT__BUTTON_MIDDLE = Gtk::GestureClick::create();

        EVENT__BUTTON_MIDDLE->set_button(
            GDK_BUTTON_MIDDLE
        );

        add_controller(
            EVENT__BUTTON_MIDDLE
        );

        // Connect events
        EVENT__BUTTON_MIDDLE->signal_pressed().connect(
            [this](int n, double x, double y)
            {
                if (!is_pinned) // @TODO match current tab condition
                {
                    action__tab_close->activate();
                }
            }
        );
}

// Actions
int Label::session_restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
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
            update(
                reinterpret_cast<const char*>(
                    sqlite3_column_text(
                        statement,
                        Database::Session::TEXT
                    )
                ),
                sqlite3_column_int(
                    statement,
                    Database::Session::IS_PINNED
                ) == 1
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
    return Database::Session::add(
        database,
        APP_BROWSER_MAIN_TAB__SESSION__ID,
        is_pinned,
        text
    );
}

void Label::pin(
    const bool & IS_PINNED
) {
    // Toggle status
    is_pinned = IS_PINNED;

    // Update widget
    if (is_pinned)
    {
        set_width_chars(
            1
        );

        set_text(
            "•" // @TODO GTK icon
        );
    }

    else
    {
        set_width_chars(
            WIDTH_CHARS
        );

        set_text(
            text
        );
    }
}

void Label::pin()
{
    pin(
        !is_pinned
    );
}

void Label::update(
    const Glib::ustring & TEXT
) {
    // Keep new value in memory (used for pin actions)
    text = TEXT;

    // Update widget
    set_tooltip_text(
        TEXT // same value for tooltip (ellipsize mode)
    );

    if (!is_pinned)
    {
        set_text(
            TEXT
        );
    }
}

void Label::update(
    const Glib::ustring & TEXT,
    const int & IS_PINNED
) {
    update(
        TEXT
    );

    pin(
        IS_PINNED
    );
}

// Database model
int Label::Database::Session::init(
    sqlite3 * database
) {
    char * error;

    return sqlite3_exec(
        database,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_label__session`
            (
                `id`        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab__session__id` INTEGER NOT NULL,
                `time`      INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `is_pinned` INTEGER NOT NULL,
                `text`      VARCHAR (1024) NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Label::Database::Session::clean(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
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
                database,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_label__session` WHERE `id` = %d
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

sqlite3_int64 Label::Database::Session::add(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
    const bool & IS_PINNED,
    const Glib::ustring & TEXT
) {
    char * error; // @TODO

    sqlite3_exec(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_label__session` (
                    `app_browser_main_tab__session__id`,
                    `is_pinned`,
                    `text`
                ) VALUES (
                    '%d',
                    '%d',
                    '%s'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID,
            IS_PINNED,
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