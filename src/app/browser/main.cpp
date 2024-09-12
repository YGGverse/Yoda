#include "main.hpp"
#include "main/tab.hpp"

using namespace app::browser;

Main::Main(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        HOMOGENEOUS
    );

    // Init components
    mainTab = Gtk::make_managed<main::Tab>(
        db,
        ACTION__UPDATE,
        ACTION__MAIN_TAB_CLOSE_ACTIVE,
        ACTION__MAIN_TAB_CLOSE_ALL,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
    );

        append(
            * mainTab
        );
}

// Actions

/// Session
int Main::restore(
    const sqlite3_int64 & APP_BROWSER__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main__session`
                        WHERE `app_browser__session__id` = %d
            )SQL",
            APP_BROWSER__SESSION__ID
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
            mainTab->restore(
                sqlite3_column_int64(
                    statement,
                    DB::SESSION::ID
                )
            );
        }
    }

    return sqlite3_finalize(
        statement
    );
};

void Main::save(
    const sqlite3_int64 & APP_BROWSER__SESSION__ID
) {
    char * error; // @TODO

    // Delete previous data
    DB::SESSION::clean(
        db,
        APP_BROWSER__SESSION__ID
    ); // @TODO run on background

    // Create new session
    const sqlite3_int64 APP_BROWSER_MAIN__SESSION__ID = DB::SESSION::add(
        db,
        APP_BROWSER__SESSION__ID
    );

    // Delegate save actions to children components
    mainTab->save(
        APP_BROWSER_MAIN__SESSION__ID
    );
};

/// Tab actions
void Main::tab_append()
{
    mainTab->append(
        _("New tab"),
        true
    );
};

void Main::tab_close()
{
    mainTab->close(
        mainTab->get_current_page()
    );
};

void Main::tab_close_left()
{
    mainTab->close_left();
};

void Main::tab_close_right()
{
    mainTab->close_right();
};

void Main::tab_close_all()
{
    mainTab->close_all();
};

//// Tab page navigation
void Main::tab_page_navigation_reload() {
    mainTab->page_navigation_reload(
        mainTab->get_current_page(), // @TODO
        true
    );
};

///// Tab page navigation history
void Main::tab_page_navigation_history_back()
{
    mainTab->page_navigation_history_back(
        mainTab->get_current_page() // @TODO
    );
};

void Main::tab_page_navigation_history_forward()
{
    mainTab->page_navigation_history_forward(
        mainTab->get_current_page() // @TODO
    );
};

/// Other
void Main::update()
{
    mainTab->update(
        mainTab->get_current_page()
    );
};

// Getters
Glib::ustring Main::get_tab_page_title()
{
    return mainTab->get_page_title(
        mainTab->get_current_page()
    );
};

Glib::ustring Main::get_tab_page_description()
{
    return mainTab->get_page_description(
        mainTab->get_current_page()
    );
};


// Database
int Main::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main__session`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser__session__id` INTEGER NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Main::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main__session`
                        WHERE `app_browser__session__id` = %d
            )SQL",
            APP_BROWSER__SESSION__ID
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
            const sqlite3_int64 APP_BROWSER_MAIN__SESSION__ID = sqlite3_column_int64(
                statement,
                DB::SESSION::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                db,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER_MAIN__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                main::Tab::DB::SESSION::clean(
                    db,
                    APP_BROWSER_MAIN__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Main::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER__SESSION__ID
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main__session` (
                    `app_browser__session__id`
                ) VALUES (
                    %d
                )
            )SQL",
            APP_BROWSER__SESSION__ID
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        db
    );
}