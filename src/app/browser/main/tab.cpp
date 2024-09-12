#include "tab.hpp"
#include "tab/label.hpp"
#include "tab/page.hpp"

using namespace app::browser::main;

Tab::Tab(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init actions
    action__update                              = ACTION__UPDATE;
    action__tab_close_active                    = ACTION__TAB_CLOSE_ACTIVE;
    action__tab_close_all                       = ACTION__MAIN_TAB_CLOSE_ALL;
    action__tab_page_navigation_history_back    = ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK;
    action__tab_page_navigation_history_forward = ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD;
    action__tab_page_navigation_reload          = ACTION__TAB_PAGE_NAVIGATION_RELOAD;

    // Init widget
    set_scrollable(
        SCROLLABLE
    );

    signal_switch_page().connect(
        [this](Gtk::Widget*, guint)
        {
            // Update window elements, e.g. tab label to header bar
            action__update->activate();
        }
    );
}

int Tab::restore(
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab__session`
                        WHERE `app_browser_main__session_id` = %d ORDER BY `page_number` ASC
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
                _("Restore"),
                sqlite3_column_int(
                    statement,
                    DB::SESSION::IS_CURRENT
                ) == 1
            );

            // Restore children components
            get_tabLabel(
                PAGE_NUMBER
            )->restore(
                sqlite3_column_int64(
                    statement,
                    DB::SESSION::ID
                )
            ); // maybe not much reasons to restore as page title in use @TODO

            get_tabPage(
                PAGE_NUMBER
            )->restore(
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
}

void Tab::save(
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    char * error; // @TODO

    // Delete previous data
    DB::SESSION::clean(
        db,
        APP_BROWSER_MAIN__SESSION__ID
    ); // @TODO run on background

    // Save current tab session
    for (int page_number = 0; page_number < get_n_pages(); page_number++)
    {
        // Create new session
        const sqlite3_int64 APP_BROWSER_MAIN_TAB__SESSION__ID = DB::SESSION::add(
            db,
            APP_BROWSER_MAIN__SESSION__ID,
            page_number,
            page_number == get_current_page()
        );

        // Delegate save actions to children components
        get_tabLabel(
            page_number
        )->save(
            APP_BROWSER_MAIN_TAB__SESSION__ID
        );

        get_tabPage(
            page_number
        )->save(
            APP_BROWSER_MAIN_TAB__SESSION__ID
        );
    }
}

// Actions
void Tab::update(
    const int & PAGE_NUMBER
) {
    // Get tab page
    const auto TAB_PAGE = get_tabPage(
        PAGE_NUMBER
    );

    // Update tab page component
    TAB_PAGE->update();

    // Update tab label component
    get_tabLabel(
        PAGE_NUMBER
    )->set_label(
        TAB_PAGE->get_title()
    );

    // Update tab actions status
    action__tab_close_active->set_enabled(
        get_n_pages() > 0
    );

    action__tab_close_all->set_enabled(
        get_n_pages() > 0
    );
}

int Tab::append(
    const Glib::ustring & LABEL_TEXT,
    const bool & IS_CURRENT
) {
    const auto TAB_PAGE = new tab::Page( // @TODO manage
        db,
        tab::Page::MIME::UNDEFINED,
        LABEL_TEXT,
        "", // @TODO restore feature

        action__update,
        action__tab_page_navigation_history_back,
        action__tab_page_navigation_history_forward,
        action__tab_page_navigation_reload
    );

    const auto TAB_LABEL = new tab::Label( // @TODO manage
        db,
        action__tab_close_active
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
    get_tabPage(
        PAGE_NUMBER
    )->navigation_reload(
        ADD_HISTORY
    );
}

void Tab::page_navigation_history_back(
    const int & PAGE_NUMBER
) {
    get_tabPage(
        PAGE_NUMBER
    )->navigation_history_back();
}

void Tab::page_navigation_history_forward(
    const int & PAGE_NUMBER
) {
    get_tabPage(
        PAGE_NUMBER
    )->navigation_history_forward();
}

// Getters
Glib::ustring Tab::get_page_title(
    const int & PAGE_NUMBER
) {
    return get_tabPage(
        PAGE_NUMBER
    )->get_title();
};

Glib::ustring Tab::get_page_description(
    const int & PAGE_NUMBER
) {
    return get_tabPage(
        PAGE_NUMBER
    )->get_description();
};

tab::Label * Tab::get_tabLabel(
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

tab::Page * Tab::get_tabPage(
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
int Tab::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab__session`
            (
                `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main__session_id` INTEGER NOT NULL,
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

int Tab::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab__session`
                        WHERE `app_browser_main_tab__session_id` = %d
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
                DB::SESSION::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                db,
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
                tab::Label::DB::SESSION::clean(
                    db,
                    APP_BROWSER_MAIN_TAB__SESSION__ID
                );

                tab::Page::DB::SESSION::clean(
                    db,
                    APP_BROWSER_MAIN_TAB__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Tab::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID,
    const int & PAGE_NUMBER,
    const bool & IS_CURRENT
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab__session` (
                    `app_browser_main__session_id`,
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
        db
    );
}