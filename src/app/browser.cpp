#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    sqlite3 * db
) {
    // Init database
    DB::SESSION::init(
        this->db = db
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

    add_action_with_parameter(
        "open",
        Glib::VARIANT_TYPE_STRING,
        [this](const Glib::VariantBase & PARAMETER)
        {
            if (PARAMETER.is_of_type(Glib::VARIANT_TYPE_STRING))
            {
                browserMain->update(
                    Glib::VariantBase::cast_dynamic<Glib::Variant<Glib::ustring>>(
                        PARAMETER
                    ).get()
                );

                browserHeader->update(
                    browserMain->get_tab_page_title(),
                    browserMain->get_tab_page_description()
                );
            }
        }
    );

    const auto ACTION__SESSION_CLEAN = add_action(
        "session_clean",
        [this]
        {
            clean();
        }
    );

    const auto ACTION__SESSION_RESTORE = add_action(
        "session_restore",
        [this]
        {
            restore();
        }
    );

    const auto ACTION__SESSION_SAVE = add_action(
        "session_save",
        [this]
        {
            save();
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
        db,
        ACTION__UPDATE,
        ACTION__TAB_CLOSE,
        ACTION__TAB_CLOSE_ALL,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__TAB_PAGE_NAVIGATION_RELOAD
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

            restore(); // last session from DB
        }
    );

    signal_close_request().connect(
        [this]
        {
            save();

            // @TODO sqlite3_close(db);

            return false;
        },
        true
    );
}

// Actions
int Browser::restore()
{
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
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
                    DB::SESSION::WIDTH
                ),
                sqlite3_column_int(
                    statement,
                    DB::SESSION::HEIGHT
                )
            );

            sqlite3_column_int(
                statement,
                DB::SESSION::IS_FULLSCREEN
            ) ? fullscreen() : unfullscreen();

            // Restore children components
            browserMain->restore(
                sqlite3_column_int(
                    statement,
                    DB::SESSION::ID
                )
            );
        }
    }

    return sqlite3_finalize(
        statement
    );
}

void Browser::clean()
{
    DB::SESSION::clean(
        db
    );

    browserMain->tab_close_all();
}

void Browser::save()
{
    char * error; // @TODO

    // Delete previous data
    DB::SESSION::clean(
        db
    ); // @TODO run on background

    // Create new session
    const sqlite3_int64 APP_BROWSER__SESSION__ID = DB::SESSION::add(
        db,
        get_width(),
        get_height(),
        is_fullscreen()
    );

    // Delegate save actions to children components
    browserMain->save(
        APP_BROWSER__SESSION__ID
    );
}

// Database
int Browser::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
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

int Browser::DB::SESSION::clean(
    sqlite3 * db
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
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
                DB::SESSION::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                db,
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
                browser::Main::DB::SESSION::clean(
                    db,
                    APP_BROWSER__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Browser::DB::SESSION::add(
    sqlite3 * db,
    const int & WIDTH,
    const int & HEIGHT,
    const bool & IS_FULLSCREEN
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
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
        db
    );
}