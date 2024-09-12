#include "page.hpp"
#include "page/content.hpp"
#include "page/navigation.hpp"

using namespace app::browser::main::tab;

Page::Page(
    sqlite3 * db,
    const MIME & MIME,
    const Glib::ustring & TITLE,
    const Glib::ustring & DESCRIPTION,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_RELOAD
) {
    // Init meta
    mime = MIME;
    title = TITLE;
    description = DESCRIPTION;
    progress_fraction = 0;

    // Init actions
    action__update = ACTION__UPDATE;

    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init widget components
    pageNavigation = Gtk::make_managed<page::Navigation>(
        this->db,
        ACTION__UPDATE,
        ACTION__PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__PAGE_NAVIGATION_RELOAD
    );

        append(
            * pageNavigation
        );

    pageContent = Gtk::make_managed<page::Content>();

        append(
            * pageContent
        );

    // Connect events
    /* activated twice on tab change @TODO
    signal_realize().connect(
        [this]
        {
            action__update->activate();
        }
    );*/
}

// Actions
int Page::restore(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    sqlite3_stmt* statement; // @TODO move to the DB model namespace

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page__session`
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
        // Use latest record as order
        while (sqlite3_step(statement) == SQLITE_ROW)
        {
            // Restore page data
            switch (
                sqlite3_column_int(
                    statement,
                    DB::SESSION::MIME
                )
            ) {
                case 0: mime = MIME::TEXT_PLAIN; break;
                case 1: mime = MIME::TEXT_GEMINI; break;
                case 2: mime = MIME::UNDEFINED; break;
                default:
                    throw _("Undefined MIME type");
            } // @TODO

            title = reinterpret_cast<const char*>(
                sqlite3_column_text(
                    statement,
                    DB::SESSION::TITLE
                )
            );

            description = reinterpret_cast<const char*>(
                sqlite3_column_text(
                    statement,
                    DB::SESSION::DESCRIPTION
                )
            );

            // Restore children components
            pageNavigation->restore(
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

int Page::save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    // Delegate save action to child components
    return pageNavigation->save(
        DB::SESSION::add(
            db,
            APP_BROWSER_MAIN_TAB__SESSION__ID,
            mime,
            title,
            description
        )
    );
}

void Page::update()
{
    // Update children components
    pageNavigation->update(
        progress_fraction
    );
}

void Page::navigation_reload(
    const bool & ADD_HISTORY
) {
    // Update navigation history?
    if (ADD_HISTORY)
    {
        pageNavigation->history_add(
            pageNavigation->get_request_text(),
            true
        );
    }

    // Reset page data
    mime = MIME::UNDEFINED;

    title = _("Update");

    description = Glib::ustring::sprintf(
        _("Begin update for %s.."),
        pageNavigation->get_request_text()
    );

    progress_fraction = 0;

    action__update->activate();

    // Connect scheme driver
    if ("file" == pageNavigation->get_request_scheme())
    {
        // @TODO
    }

    else if ("gemini" == pageNavigation->get_request_scheme())
    {
        // Create new socket connection
        GioSocketClient = Gio::SocketClient::create();

        GioSocketClient->set_tls(
            true
        );

        GioSocketClient->set_tls_validation_flags(
            Gio::TlsCertificateFlags::NO_FLAGS
        );

        GioSocketClient->set_timeout(
            15 // @TODO
        );

        GioSocketClient->connect_to_uri_async(
            pageNavigation->get_request_text(), 1965,
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                // Update
                title = _("Connect");

                description = Glib::ustring::sprintf(
                    _("Connecting to %s.."),
                    pageNavigation->get_request_host()
                );

                progress_fraction = .25;

                action__update->activate();

                try
                {
                    GioSocketConnection = GioSocketClient->connect_to_uri_finish(
                        result
                    );
                }

                catch (const Glib::Error & EXCEPTION)
                {
                    // Update
                    title = _("Oops");

                    description = EXCEPTION.what();

                    progress_fraction = 1;

                    action__update->activate();
                }

                // Connection established, begin request
                if (GioSocketConnection != nullptr)
                {
                    const Glib::ustring request = pageNavigation->get_request_text() + "\r\n";

                    GioSocketConnection->get_output_stream()->write_async(
                        request.data(),
                        request.size(),
                        [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                        {
                            // Update
                            title = _("Request");

                            description = Glib::ustring::sprintf(
                                _("Begin request to %s.."),
                                pageNavigation->get_request_host()
                            );

                            progress_fraction = .5;

                            action__update->activate();

                            // Response
                            GioSocketConnection->get_input_stream()->read_async( // | read_all_async
                                buffer,
                                sizeof(buffer) - 1,
                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                {
                                    // Update
                                    title = _("Reading");

                                    description = Glib::ustring::sprintf(
                                        _("Reading response from %s.."),
                                        pageNavigation->get_request_host()
                                    );

                                    progress_fraction = .75;

                                    action__update->activate();

                                    // Parse meta
                                    auto meta = Glib::Regex::split_simple(
                                        R"regex(^(\d+)?\s([\w]+\/[\w]+)?)regex",
                                        buffer
                                    );

                                    // Route by status code
                                    if (meta[1] == "20")
                                    {
                                        // Route by mime type or path extension
                                        if (meta[2] == "text/gemini" || Glib::str_has_suffix(pageNavigation->get_request_path(), ".gmi"))
                                        {
                                            // Update
                                            mime = MIME::TEXT_GEMINI;

                                            title = _("Parsing");

                                            description = Glib::ustring::sprintf(
                                                _("Parsing response from %s.."),
                                                pageNavigation->get_request_host()
                                            );

                                            progress_fraction = .8;

                                            action__update->activate();

                                            // Continue reading..
                                            GioSocketConnection->get_input_stream()->read_async( // | read_all_async
                                                buffer,
                                                sizeof(buffer) - 1,
                                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                                {
                                                    // Update
                                                    title = _("Done"); // @TODO page title

                                                    description = pageNavigation->get_request_host();

                                                    progress_fraction = 1;

                                                    action__update->activate();

                                                    // Set content driver
                                                    pageContent->set_text_gemini( // @TODO
                                                        buffer
                                                    );
                                                }
                                            );
                                        }

                                        else
                                        {
                                            // Update
                                            title = _("Oops");

                                            description = _("MIME type not supported");

                                            progress_fraction = 1;

                                            action__update->activate();
                                        }
                                    }

                                    else
                                    {
                                        // Update
                                        title = _("Oops");

                                        description = Glib::ustring::sprintf(
                                            _("Response code %s not supported"),
                                            meta[1]
                                        );

                                        progress_fraction = 1;

                                        action__update->activate();
                                    }

                                    GioSocketConnection->close();
                                }
                            );
                        }
                    );
                }
            }
        );
    }

    // Scheme not found but host provided, redirect to gemini://
    else if (pageNavigation->get_request_scheme().empty() && !pageNavigation->get_request_host().empty())
    {
        pageNavigation->set_request_text(
            "gemini://" + pageNavigation->get_request_text()
        );

        navigation_reload(
            false
        );
    }

    else
    {
        // @TODO search request
    }
}

void Page::navigation_history_back()
{
    Glib::ustring request;

    if (pageNavigation->try_history_back(request, true))
    {
        pageNavigation->set_request_text(
            request
        );

        navigation_reload(
            false
        );
    }
}

void Page::navigation_history_forward()
{
    Glib::ustring request;

    if (pageNavigation->try_history_forward(request, true))
    {
        pageNavigation->set_request_text(
            request
        );

        navigation_reload(
            false
        );
    }
}

// Getters
Page::MIME Page::get_mime()
{
    return mime;
}

Glib::ustring Page::get_title()
{
    return title;
}

Glib::ustring Page::get_description()
{
    return description;
}

// Database model
int Page::DB::SESSION::init(
    sqlite3 * db
) {
    char * error;

    return sqlite3_exec(
        db,
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_main_tab_page__session`
            (
                `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab__session__id` INTEGER NOT NULL,
                `time`        INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `mime`        INTEGER NOT NULL,
                `title`       VARCHAR(1024) NOT NULL,
                `description` VARCHAR(1024) NOT NULL
            )
        )SQL",
        nullptr,
        nullptr,
        &error
    );
}

int Page::DB::SESSION::clean(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page__session` WHERE `app_browser_main_tab__session__id` = %d
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
            const sqlite3_int64 APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID = sqlite3_column_int64(
                statement,
                DB::SESSION::ID
            );

            // Delete record
            const int EXEC_STATUS = sqlite3_exec(
                db,
                Glib::ustring::sprintf(
                    R"SQL(
                        DELETE FROM `app_browser_main_tab_page__session` WHERE `id` = %d
                    )SQL",
                    APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
                ).c_str(),
                nullptr,
                nullptr,
                &error
            );

            // Delegate children dependencies cleanup
            if (EXEC_STATUS == SQLITE_OK)
            {
                page::Navigation::DB::SESSION::clean(
                    db,
                    APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
                );
            }
        }
    }

    return sqlite3_finalize(
        statement
    );
}

sqlite3_int64 Page::DB::SESSION::add(
    sqlite3 * db,
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
    const Page::MIME & MIME,
    const Glib::ustring & TITLE,
    const Glib::ustring & DESCRIPTION
) {
    char * error; // @TODO

    sqlite3_exec(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                INSERT INTO `app_browser_main_tab_page__session` (
                    `app_browser_main_tab__session__id`,
                    `mime`,
                    `title`,
                    `description`
                ) VALUES (
                    '%d',
                    '%d',
                    '%s',
                    '%s'
                )
            )SQL",
            APP_BROWSER_MAIN_TAB__SESSION__ID,
            MIME,
            TITLE,
            DESCRIPTION
        ).c_str(),
        nullptr,
        nullptr,
        &error
    );

    return sqlite3_last_insert_rowid(
        db
    );
}