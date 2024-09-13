#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::page::navigation;

History::History(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init widget
    add_css_class(
        "linked" // merge children elements
    );

    historyBack = Gtk::make_managed<history::Back>(
        ACTION__HISTORY_BACK
    );

        append(
            * historyBack
        );

    historyForward = Gtk::make_managed<history::Forward>(
        ACTION__HISTORY_FORWARD
    );

        append(
            * historyForward
        );
}

// Actions
void History::add(
    const Glib::ustring & REQUEST,
    const bool & UPDATE_MEMORY_INDEX
) {
    memory.push_back(
        {
            REQUEST,
            std::time(
                nullptr
            )
        }
    );

    if (UPDATE_MEMORY_INDEX)
    {
        index = memory.size() - 1;
    }
}

int History::restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation_history__session`
                        WHERE `app_browser_main_tab_page_navigation__session__id` = %d
                        ORDER BY `id` DESC
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
        ).c_str(),
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        // Reset current memory index
        index = -1;

        // Cleanup memory vector
        memory.clear();

        // Restore memory from database records
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore
            memory.push_back(
                {
                    reinterpret_cast<const char*>(
                        sqlite3_column_text(
                            statement,
                            DB::SESSION::REQUEST
                        )
                    ),
                    sqlite3_column_int(
                        statement,
                        DB::SESSION::TIME
                    )
                }
            );

            if (sqlite3_column_int(statement, DB::SESSION::IS_CURRENT) == 1)
            {
                index = memory.size() - 1;
            }

            // Restore children components here (on available)
        }

        // Navigate to latest memory index on current value still not found in database
        if (index == -1)
        {
            index = memory.size() - 1;
        }
    }

    return sqlite3_finalize(
        statement
    );
}

void History::save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    // Delete previous records for session
    DB::SESSION::clean(
        db,
        APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
    );

    // Add new records
    int offset = -1; for (const auto & MEMORY : memory)
    {
        DB::SESSION::add(
            db,
            APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
            MEMORY.time,
            MEMORY.request,
            ++offset == index // is current
        );
    }
}

void History::update()
{
    Memory match;

    historyBack->update(
        try_back(
            match,
            false
        )
    );

    historyForward->update(
        try_forward(
            match,
            false
        )
    );
}

bool History::try_back(
    Memory & match,
    const bool & UPDATE_MEMORY_INDEX
) {
    try
    {
        match = memory.at(
            index - 1
        );

        if (UPDATE_MEMORY_INDEX)
        {
            index--;
        }

        return true;
    }

    catch (std::out_of_range)
    {
        return false;
    }
}

bool History::try_forward(
    Memory & match,
    const bool & UPDATE_MEMORY_INDEX
) {
    try
    {
        match = memory.at(
            index + 1
        );

        if (UPDATE_MEMORY_INDEX)
        {
            index++;
        }

        return true;
    }

    catch (std::out_of_range)
    {
        return false;
    }
}

// Database model
int History::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_page_navigation_history__session`
            (
                `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab_page_navigation__session__id` INTEGER NOT NULL,
                `time`       INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `request`    VARCHAR (1024) NOT NULL,
                `is_current` INTEGER NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int History::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation_history__session`
                        WHERE `app_browser_main_tab_page_navigation__session__id` = %d
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
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
                        DELETE FROM `app_browser_main_tab_page_navigation_history__session` WHERE `id` = %d
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

sqlite3_int64 History::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
    const int & TIME,
    const Glib::ustring & REQUEST,
    const bool & IS_CURRENT
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_page_navigation_history__session` (
                    `app_browser_main_tab_page_navigation__session__id`,
                    `time`,
                    `request`,
                    `is_current`
                ) VALUES (
                    '%d',
                    '%d',
                    '%s',
                    '%d'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
            TIME,
            REQUEST,
            IS_CURRENT
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        db
    );
}