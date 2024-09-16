#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    sqlite3 * database
) {
    // Init database
    Database::Session::init(
        this->database = database
    );

    // Init window actions
    const auto ACTION__UPDATE = add_action(
        "update",
        [this]
        {
            browserMain->update();

            browserHeader->update(
                browserMain->get_tab_page_title(),
                browserMain->get_tab_page_description()
            );
        }
    );

    const auto ACTION__SESSION_CLEAN = add_action(
        "session_clean",
        [this]
        {
            session_clean();
        }
    );

    const auto ACTION__SESSION_RESTORE = add_action(
        "session_restore",
        [this]
        {
            session_restore();
        }
    );

    const auto ACTION__SESSION_SAVE = add_action(
        "session_save",
        [this]
        {
            session_save();
        }
    );

    const auto ACTION__TOOLS_DEBUG = add_action(
        "tools_debug",
        [this]
        {
            // @TODO https://gitlab.gnome.org/GNOME/gtkmm/-/commit/5f3b82537d3daad7bda59dd01e719788070f4b6c
            gtk_window_set_interactive_debugging(
                true
            );
        }
    );

    const auto ACTION__TAB_APPEND = add_action(
        "tab_append",
        [this]
        {
            browserMain->tab_append();
        }
    );

    const auto ACTION__TAB_CLOSE = add_action(
        "tab_close",
        [this]
        {
            browserMain->tab_close();
        }
    );

        ACTION__TAB_CLOSE->set_enabled(
            false
        );

    add_action(
        "tab_close_left",
        [this]
        {
            browserMain->tab_close_left();
        }
    )->set_enabled(
        false
    );

    add_action(
        "tab_close_right",
        [this]
        {
            browserMain->tab_close_right();
        }
    )->set_enabled(
        false
    );

    const auto ACTION__TAB_CLOSE_ALL = add_action(
        "tab_close_all",
        [this]
        {
            browserMain->tab_close_all();
        }
    );

        ACTION__TAB_CLOSE_ALL->set_enabled(
            false
        );

    const auto ACTION__TAB_PAGE_NAVIGATION_RELOAD = add_action(
        "tab_page_navigation_reload",
        [this]
        {
            browserMain->tab_page_navigation_reload();
        }
    );

        ACTION__TAB_PAGE_NAVIGATION_RELOAD->set_enabled(
            false
        );

    const auto ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK = add_action(
        "tab_page_navigation_history_back",
        [this]
        {
            browserMain->tab_page_navigation_history_back();
        }
    );

        ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK->set_enabled(
            false
        );

    const auto ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD = add_action(
        "tab_page_navigation_history_forward",
        [this]
        {
            browserMain->tab_page_navigation_history_forward();
        }
    );

        ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD->set_enabled(
            false
        );

    const auto ACTION__QUIT = add_action(
        "quit",
        [this]
        {
            close();
        }
    );

    // Init widget
    set_title(
        _("Yoda")
    );

    set_default_size(
        WIDTH,
        HEIGHT
    );

    IS_FULLSCREEN ? fullscreen() : unfullscreen();

    // Init components
    browserHeader = Gtk::make_managed<browser::Header>(
        ACTION__TOOLS_DEBUG,
        ACTION__QUIT,
        ACTION__SESSION_CLEAN,
        ACTION__SESSION_RESTORE,
        ACTION__SESSION_SAVE,
        ACTION__TAB_APPEND,
        ACTION__TAB_CLOSE,
        ACTION__TAB_CLOSE_ALL,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__TAB_PAGE_NAVIGATION_RELOAD
    );

        set_titlebar(
            * browserHeader
        );

    browserMain = Gtk::make_managed<browser::Main>(
        database,
        ACTION__TAB_CLOSE,
        ACTION__TAB_CLOSE_ALL,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__TAB_PAGE_NAVIGATION_RELOAD,
        ACTION__UPDATE
    );

        set_child(
            * browserMain
        );

    // Connect signals
    signal_realize().connect(
        [this]
        {
            const auto APP = get_application();

            APP->set_accel_for_action(
                "win.debug",
                "<Primary>i"
            );

            APP->set_accel_for_action(
                "win.tab_append",
                "<Primary>t"
            );

            APP->set_accel_for_action(
                "win.tab_close",
                "<Primary>Escape"
            );

            APP->set_accel_for_action(
                "win.tab_page_navigation_reload",
                "<Primary>r"
            );

            APP->set_accel_for_action(
                "win.tab_page_navigation_history_back",
                "<Primary>Left"
            );

            APP->set_accel_for_action(
                "win.tab_page_navigation_history_forward",
                "<Primary>Right"
            );

            APP->set_accel_for_action(
                "win.quit",
                "<Primary>q"
            );

            session_restore(); // last session from Database
        }
    );

    signal_close_request().connect(
        [this]
        {
            session_save();

            // @TODO sqlite3_close(database);

            return false;
        },
        true
    );
}

// Actions
int Browser::session_restore()
{
    sqlite3_stmt* statement; // @TODO move to the Database model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        R"SQL(
            SELECT * FROM `app_browser__session`
        )SQL",
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore widget settings
            set_default_size(
                sqlite3_column_int(
                    statement,
                    Database::Session::WIDTH
                ),
                sqlite3_column_int(
                    statement,
                    Database::Session::HEIGHT
                )
            );

            sqlite3_column_int(
                statement,
                Database::Session::IS_FULLSCREEN
            ) == 1 ? fullscreen() : unfullscreen();

            // Restore children components
            browserMain->session_restore(
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

void Browser::session_clean()
{
    Database::Session::clean(
        database
    );

    browserMain->tab_close_all();
}

void Browser::session_save()
{
    char * error; // @TODO

    // Delete previous data
    Database::Session::clean(
        database
    ); // @TODO run on background

    // Create new session
    const sqlite3_int64 APP_BROWSER__SESSION__ID = Database::Session::add(
        database,
        get_width(),
        get_height(),
        is_fullscreen()
    );

    // Delegate save actions to children components
    browserMain->session_save(
        APP_BROWSER__SESSION__ID
    );
}

// Database
int Browser::Database::Session::init(
    sqlite3 * database
) {
    char * error;

    return sqlite3_exec(
        database,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser__session`
            (
                `id`            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`          INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `width`         INTEGER NOT NULL,
                `height`        INTEGER NOT NULL,
                `is_fullscreen` INTEGER NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Browser::Database::Session::clean(
    sqlite3 * database
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        R"SQL(
            SELECT * FROM `app_browser__session`
        )SQL",
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            const sqlite3_int64 APP_BROWSER__SESSION__ID = sqlite3_column_int64(
                statement,
                Database::Session::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                database,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                browser::Main::Database::Session::clean(
                    database,
                    APP_BROWSER__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Browser::Database::Session::add(
    sqlite3 * database,
    const int & WIDTH,
    const int & HEIGHT,
    const bool & IS_FULLSCREEN
) {
    char * error; // @TODO

    sqlite3_exec(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser__session` (
                    `width`,
                    `height`,
                    `is_fullscreen`
                ) VALUES (
                    %d,
                    %d,
                    %d
                )
            )SQL",
            WIDTH,
            HEIGHT,
            IS_FULLSCREEN ? 1 : 0
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        database
    );
}