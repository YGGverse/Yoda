#include "navigation.hpp"
#include "navigation/base.hpp"
#include "navigation/bookmark.hpp"
#include "navigation/history.hpp"
#include "navigation/request.hpp"
#include "navigation/reload.hpp"

using namespace app::browser::main::tab::page;

Navigation::Navigation(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_RELOAD
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init container
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_spacing(
        SPACING
    );

    set_margin_top(
        MARGIN
    );

    set_margin_start(
        MARGIN
    );

    set_margin_end(
        MARGIN
    );

    set_margin_bottom(
        MARGIN
    );

    // Init components
    navigationBase = Gtk::make_managed<navigation::Base>();

        append(
            * navigationBase
        );

    navigationHistory = Gtk::make_managed<navigation::History>(
        ACTION__NAVIGATION_HISTORY_BACK,
        ACTION__NAVIGATION_HISTORY_FORWARD
    );

        append(
            * navigationHistory
        );

    navigationReload = Gtk::make_managed<navigation::Reload>(
        ACTION__NAVIGATION_RELOAD
    );

        append(
            * navigationReload
        );

    navigationRequest = Gtk::make_managed<navigation::Request>(
        db,
        ACTION__REFRESH,
        ACTION__NAVIGATION_RELOAD
    );

        append(
            * navigationRequest
        );

    navigationBookmark = Gtk::make_managed<navigation::Bookmark>();

        append(
            * navigationBookmark
        );
}

// Actions
void Navigation::update(
    const double & PROGRESS_FRACTION
) {
    // Toggle base button sensibility
    navigationBase->set_sensitive(
        !navigationRequest->get_host().empty() && !navigationRequest->get_path().empty()
    );

    // Refresh history widget
    navigationHistory->update();

    // Toggle update button sensibility
    navigationReload->update(
        navigationRequest->get_text_length() > 0
    );

    // Refresh request area (with progressbar)
    navigationRequest->update(
        PROGRESS_FRACTION
    );
}

int Navigation::restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation__session`
                        WHERE `app_browser_main_tab_page__session__id` = %d
                        ORDER BY `id` DESC LIMIT 1
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
        ).c_str(),
        -1,
        SQLITE_PREPARE_NORMALIZE,
        &statement,
        nullptr
    );

    if (PREPARE_STATUS == SQLITE_OK)
    {
        // Use latest record as order
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore children components
            navigationRequest->restore(
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

int Navigation::save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
) {
    return navigationRequest->save(
        DB::SESSION::add(
            db,
            APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
        )
    );
}

void Navigation::history_add(
    const Glib::ustring & REQUEST,
    const bool & UPDATE_MEMORY_INDEX
) {
    navigationHistory->add(
        REQUEST,
        UPDATE_MEMORY_INDEX
    );
}

// Actionable getters
bool Navigation::try_history_back(
    Glib::ustring & request,
    const bool & UPDATE_MEMORY_INDEX
) {
    navigation::History::Memory match;

    if (navigationHistory->try_back(match, UPDATE_MEMORY_INDEX))
    {
        request = match.request;

        return true;
    }

    return false;
}

bool Navigation::try_history_forward(
    Glib::ustring & request,
    const bool & UPDATE_MEMORY_INDEX
) {
    navigation::History::Memory match;

    if (navigationHistory->try_forward(match, UPDATE_MEMORY_INDEX))
    {
        request = match.request;

        return true;
    }

    return false;
}

// Getters @TODO &
Glib::ustring Navigation::get_request_text()
{
    return navigationRequest->get_text();
}

Glib::ustring Navigation::get_request_scheme()
{
    return navigationRequest->get_scheme();
}

Glib::ustring Navigation::get_request_host()
{
    return navigationRequest->get_host();
}

Glib::ustring Navigation::get_request_path()
{
    return navigationRequest->get_path();
}

Glib::ustring Navigation::get_request_query()
{
    return navigationRequest->get_query();
}

Glib::ustring Navigation::get_request_port()
{
    return navigationRequest->get_port();
}

// Setters
void Navigation::set_request_text(
    const Glib::ustring & VALUE
) {
    navigationRequest->set_text(
        VALUE
    );
}

// Database model
int Navigation::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_page_navigation__session`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab_page__session__id` INTEGER NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Navigation::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation__session` WHERE `app_browser_main_tab_page__session__id` = %d
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
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
            const sqlite3_int64 APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID = sqlite3_column_int64(
                statement,
                DB::SESSION::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                db,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_page_navigation__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                navigation::Request::DB::SESSION::clean(
                    db,
                    APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Navigation::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_page_navigation__session` (
                    `app_browser_main_tab_page__session__id`
                ) VALUES (
                    '%d'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        db
    );
}