#include "tab.hpp"
#include "tab/label.hpp"
#include "tab/page.hpp"

using namespace app::browser::main;

Tab::Tab(
    sqlite3 * database,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
) {
    // Init database
    Database::Session::init(
        this->database = database
    );

    // Init actions
    action__close           = ACTION__CLOSE;
    action__close_all       = ACTION__CLOSE_ALL;
    action__history_back    = ACTION__HISTORY_BACK;
    action__history_forward = ACTION__HISTORY_FORWARD;
    action__reload          = ACTION__RELOAD;
    action__update          = ACTION__UPDATE;

    // Init widget
    set_scrollable(
        SCROLLABLE
    );

    // Init additional controllers
    const auto EVENT__BUTTON_PRIMARY = Gtk::GestureClick::create();

        /* use defaults
        EVENT__BUTTON_PRIMARY->set_button(
            GDK_BUTTON_PRIMARY
        );*/

    add_controller(
        EVENT__BUTTON_PRIMARY
    );

    // Init events
    EVENT__BUTTON_PRIMARY->signal_pressed().connect(
        [this](int n, double x, double y)
        {
            if (n == 2) // double click
            {
                tabLabel(
                    get_current_page()
                )->pin();
            }
        }
    );

    signal_switch_page().connect(
        [this](Gtk::Widget*, guint)
        {
            // Update window elements, e.g. tab label to header bar
            action__update->activate();
        }
    );
}

int Tab::session_restore(
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the Database model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab__session`
                        WHERE `app_browser_main__session__id` = %d ORDER BY `page_number` ASC
            )SQL",
            APP_BROWSER_MAIN__SESSION__ID
        ).c_str(),
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        close_all();

        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            const int PAGE_NUMBER = append(
                sqlite3_column_int(
                    statement,
                    Database::Session::IS_CURRENT
                ) == 1
            );

            // Restore children components
            tabLabel(
                PAGE_NUMBER
            )->session_restore(
                sqlite3_column_int64(
                    statement,
                    Database::Session::ID
                )
            ); // maybe not much reasons to restore as page title in use @TODO

            tabPage(
                PAGE_NUMBER
            )->session_restore(
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

void Tab::session_save(
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    char * error; // @TODO

    // Delete previous data
    Database::Session::clean(
        database,
        APP_BROWSER_MAIN__SESSION__ID
    ); // @TODO run on background

    // Save current tab session
    for (int page_number = 0; page_number < get_n_pages(); page_number++)
    {
        // Create new session
        const sqlite3_int64 APP_BROWSER_MAIN_TAB__SESSION__ID = Database::Session::add(
            database,
            APP_BROWSER_MAIN__SESSION__ID,
            page_number,
            page_number == get_current_page()
        );

        // Delegate save actions to children components
        tabLabel(
            page_number
        )->session_save(
            APP_BROWSER_MAIN_TAB__SESSION__ID
        );

        tabPage(
            page_number
        )->session_save(
            APP_BROWSER_MAIN_TAB__SESSION__ID
        );
    }
}

// Actions
void Tab::update(
    const int & PAGE_NUMBER
) {
    // Get tab page
    const auto TAB_PAGE = tabPage(
        PAGE_NUMBER
    );

    // Update tab page component
    TAB_PAGE->update();

    // Update tab label component
    tabLabel(
        PAGE_NUMBER
    )->update(
        TAB_PAGE->get_title()
    );

    // Update tab actions status
    action__close->set_enabled(
        get_n_pages() > 0
    );

    action__close_all->set_enabled(
        get_n_pages() > 0
    );
}

int Tab::append(
    const bool & IS_CURRENT
) {
    const auto TAB_PAGE = new tab::Page( // @TODO manage
        database,
        action__history_back,
        action__history_forward,
        action__reload,
        action__update
    );

    const auto TAB_LABEL = new tab::Label( // @TODO manage
        database,
        action__close
    );

    const int PAGE_NUMBER = append_page(
        * TAB_PAGE,
        * TAB_LABEL
    );

    set_tab_reorderable(
        * TAB_PAGE,
        REORDERABLE
    );

    if (IS_CURRENT)
    {
        set_current_page(
            PAGE_NUMBER
        );
    }

    update(
        PAGE_NUMBER
    );

    return PAGE_NUMBER;
};

void Tab::pin(
    const int & PAGE_NUMBER
) {
    tabLabel(
        PAGE_NUMBER
    )->pin();
}

void Tab::close(
    const int & PAGE_NUMBER
) {
    remove_page(
        PAGE_NUMBER
    );

    // @TODO cleanup memory ot use managed children widgets
    // @TODO fix GtkGizmo reported min height, but sizes must be >= 0
}

void Tab::close_left()
{} // @TODO

void Tab::close_right()
{} // @TODO

void Tab::close_all()
{
    while (0 <= get_current_page())
    {
        close(
            -1 // last
        );
    }
}

void Tab::page_navigation_reload(
    const int & PAGE_NUMBER,
    const bool & ADD_HISTORY
) {
    tabPage(
        PAGE_NUMBER
    )->navigation_reload(
        ADD_HISTORY
    );
}

void Tab::page_navigation_history_back(
    const int & PAGE_NUMBER
) {
    tabPage(
        PAGE_NUMBER
    )->navigation_history_back();
}

void Tab::page_navigation_history_forward(
    const int & PAGE_NUMBER
) {
    tabPage(
        PAGE_NUMBER
    )->navigation_history_forward();
}

// Getters
Glib::ustring Tab::get_page_title(
    const int & PAGE_NUMBER
) {
    return tabPage(
        PAGE_NUMBER
    )->get_title();
};

Glib::ustring Tab::get_page_description(
    const int & PAGE_NUMBER
) {
    return tabPage(
        PAGE_NUMBER
    )->get_description();
};

tab::Label * Tab::tabLabel(
    const int & PAGE_NUMBER
) {
    // Get page pointer to find label widget
    const auto PAGE = get_nth_page(
        PAGE_NUMBER
    );

        if (PAGE == nullptr)
        {
            throw _("Page not found!");  // @TODO
        }

    // Get label widget by page pointer
    const auto LABEL = get_tab_label(
        * PAGE
    );

        if (LABEL == nullptr)
        {
            throw _("Label not found!"); // @TODO
        }

    // Downcast
    const auto TAB_LABEL = dynamic_cast<tab::Label*>(
        LABEL
    );

        if (TAB_LABEL == nullptr)
        {
            throw _("Tab label not found!"); // @TODO
        }

    return TAB_LABEL;
}

tab::Page * Tab::tabPage(
    const int & PAGE_NUMBER
) {
    // Get page widget
    const auto PAGE = get_nth_page(
        PAGE_NUMBER
    );

        if (PAGE == nullptr)
        {
            throw _("Page not found!"); // @TODO
        }

    // Downcast
    const auto TAB_PAGE = dynamic_cast<tab::Page*>(
        PAGE
    );

        if (TAB_PAGE == nullptr)
        {
            throw _("Tab page not found!"); // @TODO
        }

    return TAB_PAGE;
}

// Database
int Tab::Database::Session::init(
    sqlite3 * database
) {
    char * error;

    return sqlite3_exec(
        database,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab__session`
            (
                `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main__session__id` INTEGER NOT NULL,
                `time`        INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `page_number` INTEGER NOT NULL,
                `is_current`  INTEGER NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Tab::Database::Session::clean(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab__session`
                        WHERE `app_browser_main__session__id` = %d
            )SQL",
            APP_BROWSER_MAIN__SESSION__ID
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
            const sqlite3_int64 APP_BROWSER_MAIN_TAB__SESSION__ID = sqlite3_column_int64(
                statement,
                Database::Session::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                database,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER_MAIN_TAB__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                tab::Label::Database::Session::clean(
                    database,
                    APP_BROWSER_MAIN_TAB__SESSION__ID
                );

                tab::Page::Database::Session::clean(
                    database,
                    APP_BROWSER_MAIN_TAB__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Tab::Database::Session::add(
    sqlite3 * database,
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID,
    const int & PAGE_NUMBER,
    const bool & IS_CURRENT
) {
    char * error; // @TODO

    sqlite3_exec(
        database,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab__session` (
                    `app_browser_main__session__id`,
                    `page_number`,
                    `is_current`
                ) VALUES (
                    %d,
                    %d,
                    %d
                )
            )SQL",
            APP_BROWSER_MAIN__SESSION__ID,
            PAGE_NUMBER,
            IS_CURRENT ? 1 : 0
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        database
    );
}