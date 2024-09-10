#include "request.hpp"

using namespace app::browser::main::tab::page::navigation;

// Construct
Request::Request(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init actions
    action__refresh = ACTION__REFRESH;
    action__update  = ACTION__UPDATE;

    // Init widget
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        HEXPAND
    );

    set_progress_pulse_step(
        PROGRESS_PULSE_STEP
    );

    // Connect events
    signal_changed().connect(
        [this]
        {
            parse();

            action__refresh->activate();
        }
    );

    signal_activate().connect(
        [this]
        {
            parse();

            action__update->activate();
        }
    );
}

// Actions
void Request::refresh(
    const double & PROGRESS_FRACTION
) {
    // Update progress
    progress_fraction = PROGRESS_FRACTION;

    // Animate progress function
    Glib::signal_timeout().connect(
        [this]() -> bool
        {
            double current_progress_fraction = get_progress_fraction();

            // Animation in progress
            if (current_progress_fraction < progress_fraction)
            {
                set_progress_fraction(
                    current_progress_fraction + PROGRESS_PULSE_STEP
                );

                return true; // continue
            }

            // 100% of value, reset
            set_progress_fraction(
                progress_fraction = 0
            );

            return false; // stop
        },
        PROGRESS_ANIMATION_TIME
    );
}

int Request::save(
    const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID
) {
    return DB::SESSION::add(
        db,
        DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID,
        get_text()
    );
}

void Request::parse()
{
    auto match = Glib::Regex::split_simple(
        R"regex(^((\w+)?:\/\/)?([^:\/]+)?(:(\d+)?)?([^\?$]+)?(\?(.*)?)?)regex",
        get_text()
    );

    scheme = "";
    host   = "";
    port   = "";
    path   = "";
    query  = "";

    int index = 0;

    for (const Glib::ustring & VALUE : match)
    {
        switch (index)
        {
            case 2: scheme = VALUE; break;
            case 3: host   = VALUE; break;
            case 5: port   = VALUE; break;
            case 6: path   = VALUE; break;
            case 8: query  = VALUE; break;
        }

        index++;
    }
}

// Getters
Glib::ustring Request::get_scheme()
{
    return scheme;
}

Glib::ustring Request::get_host()
{
    return host;
}

Glib::ustring Request::get_port()
{
    return port;
}

Glib::ustring Request::get_path()
{
    return path;
}

Glib::ustring Request::get_query()
{
    return path;
}

// Database model
int Request::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_page_navigation_request__session`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab_page_navigation__session_id` INTEGER NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `text` VARCHAR (1024) NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Request::DB::SESSION::clean(
    sqlite3 * db,
    const int & DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation_request__session`
                        WHERE `app_browser_main_tab_page_navigation__session_id` = %d
            )SQL",
            DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID
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
                        DELETE FROM `app_browser_main_tab_page_navigation_request__session` WHERE `id` = %d
                    )SQL",
                    sqlite3_column_int(
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

sqlite3_int64 Request::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID,
    const Glib::ustring & TEXT
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_page_navigation_request__session` (
                    `app_browser_main_tab_page_navigation__session_id`,
                    `text`
                ) VALUES (
                    '%d',
                    '%s'
                )
            )SQL",
            DB__APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID,
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