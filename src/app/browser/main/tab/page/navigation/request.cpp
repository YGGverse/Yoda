#include "request.hpp"

using namespace app::browser::main::tab::page::navigation;

// Construct
Request::Request(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD
) {
    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init actions
    action__update = ACTION__UPDATE;
    action__reload = ACTION__RELOAD;

    // Init extras
    progress_fraction = 0;

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
            action__update->activate();
        }
    );

    signal_activate().connect(
        [this]
        {
            action__reload->activate();
        }
    );
}

// Actions
void Request::update(
    const double & PROGRESS_FRACTION
) {
    // Update progress
    progress_fraction = PROGRESS_FRACTION;

    // Reset previous connection
    if (progress_connection.connected())
    {
        progress_connection.disconnect();
    }

    // Animate progress function
    progress_connection = Glib::signal_timeout().connect(
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

void Request::update(
    const Glib::ustring & TEXT,
    const double & PROGRESS_FRACTION
) {
    set_text(
        TEXT
    );

    update(
        PROGRESS_FRACTION
    );
}

int Request::restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation_request__session`
                        WHERE `app_browser_main_tab_page_navigation__session__id` = %d
                        ORDER BY `id` DESC LIMIT 1
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
        // Restore entry value from latest database record
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore widget data
            set_text(
                reinterpret_cast<const char*>(
                    sqlite3_column_text(
                        statement,
                        DB::SESSION::TEXT
                    )
                )
            );

            // Restore children components here (on available)
        }
    }

    return sqlite3_finalize(
        statement
    );
}

int Request::save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    // Delete previous records
    DB::SESSION::clean(
        db,
        APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
    );

    // Add new record
    return DB::SESSION::add(
        db,
        APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
        get_text()
    );
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
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab_page_navigation__session__id` INTEGER NOT NULL,
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
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page_navigation_request__session`
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
                        DELETE FROM `app_browser_main_tab_page_navigation_request__session` WHERE `id` = %d
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

sqlite3_int64 Request::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
    const Glib::ustring & TEXT
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_page_navigation_request__session` (
                    `app_browser_main_tab_page_navigation__session__id`,
                    `text`
                ) VALUES (
                    '%d',
                    '%s'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
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