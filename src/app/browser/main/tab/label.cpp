#include "label.hpp"
#include "label/pin.hpp"
#include "label/title.hpp"

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
    is_pinned = false;

    // Init widget
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_halign(
        Gtk::Align::CENTER
    );

    // Init components
    labelPin = Gtk::make_managed<label::Pin>();

        labelPin->hide();

        append(
            * labelPin
        );

    labelTitle = Gtk::make_managed<label::Title>(
        database
    );

        set_tooltip_text(
            labelTitle->get_text()
        );

        append(
            * labelTitle
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
                    update(
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
                sqlite3_column_int(
                    statement,
                    Database::Session::IS_PINNED
                ) == 1
            );

            // Restore children components
            labelTitle->session_restore(
                sqlite3_column_int64(
                    statement,
                    Database::Session::ID
                )
            );
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Label::session_save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    // Create new session
    const sqlite3_int64 APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID = Database::Session::add(
        database,
        APP_BROWSER_MAIN_TAB__SESSION__ID,
        is_pinned
    );

    // Delegate save action to child components
    labelTitle->session_save(
        APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
    );

    // Return ID
    return APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID;
}

void Label::pin()
{
    update(
        !is_pinned
    );
}

void Label::update(
    const bool & IS_PINNED
) {
    // Toggle status
    is_pinned = IS_PINNED;

    if (is_pinned)
    {
        labelPin->show();
        labelTitle->hide();
    }

    else
    {
        labelPin->hide();
        labelTitle->show();
    }
}

void Label::update(
    const Glib::ustring & TITLE
) {
    set_tooltip_text(
        TITLE
    );

    labelTitle->update(
        TITLE
    );
}

void Label::update(
    const Glib::ustring & TITLE,
    const int & IS_PINNED
) {
    update(
        TITLE
    );

    update(
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
                `is_pinned` INTEGER NOT NULL
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
            const sqlite3_int64 APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID = sqlite3_column_int64(
                statement,
                Database::Session::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                database,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_label__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                label::Title::Database::Session::clean(
                    database,
                    APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
                );
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
    const bool & IS_PINNED
) {
    char * error; // @TODO

    sqlite3_exec(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_label__session` (
                    `app_browser_main_tab__session__id`,
                    `is_pinned`
                ) VALUES (
                    %d,
                    %d
                )
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID,
            IS_PINNED
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        database
    );
}