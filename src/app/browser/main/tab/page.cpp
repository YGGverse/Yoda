#include "page.hpp"
#include "page/content.hpp"
#include "page/navigation.hpp"

using namespace app::browser::main::tab;

Page::Page(
    sqlite3 * db,
    const MIME & MIME,
    const Glib::ustring & TITLE,
    const Glib::ustring & DESCRIPTION,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_UPDATE
) {
    // Init meta
    mime = MIME;
    title = TITLE;
    description = DESCRIPTION;
    progress_fraction = 0;

    // Init actions
    action__refresh = ACTION__REFRESH;

    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init components
    pageNavigation = Gtk::make_managed<page::Navigation>(
        this->db,
        ACTION__REFRESH,
        ACTION__PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__PAGE_NAVIGATION_UPDATE
    );

        append(
            * pageNavigation
        );

    pageContent = Gtk::make_managed<page::Content>();

        append(
            * pageContent
        );

    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Connect events
    signal_realize().connect(
        [this]
        {
            // Refresh parent window
            action__refresh->activate();
        }
    );
}

// Actions
void Page::refresh()
{
    pageNavigation->refresh(
        progress_fraction
    );
}

int Page::save(
    const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB__SESSION_ID
) {
    // Delegate save action to child components
    return pageNavigation->save(
        DB::SESSION::add(
            db,
            DB__APP_BROWSER_MAIN_TAB__SESSION_ID,
            mime,
            title,
            description
        )
    );
}

void Page::update(
    const enum MIME & MIME,
    const Glib::ustring & TITLE,
    const Glib::ustring & DESCRIPTION,
    const double & PROGRESS_FRACTION
) {
    // Refresh page data
    mime              = MIME;
    title             = TITLE;
    description       = DESCRIPTION;
    progress_fraction = PROGRESS_FRACTION;

    // Refresh parent window
    action__refresh->activate();
}

void Page::navigation_update(
    const bool & ADD_HISTORY
) {
    // Update navigation history
    if (ADD_HISTORY)
    {
        pageNavigation->history_add(
            pageNavigation->get_request_text(),
            true
        );
    }

    // Update page extras
    update(
        MIME::UNDEFINED,
        _("Update"),
        Glib::ustring::sprintf(
            _("Begin update for %s.."),
            pageNavigation->get_request_text()
        ),
        0
    );

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
                update(
                    MIME::UNDEFINED,
                    _("Connect"),
                    Glib::ustring::sprintf(
                        _("Connecting to %s.."),
                        pageNavigation->get_request_host()
                    ),
                    .25
                );

                try
                {
                    GioSocketConnection = GioSocketClient->connect_to_uri_finish(
                        result
                    );
                }

                catch (const Glib::Error & EXCEPTION)
                {
                    update(
                        MIME::UNDEFINED,
                        _("Oops"),
                        EXCEPTION.what(),
                        1
                    );
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
                            update(
                                MIME::UNDEFINED,
                                _("Request"),
                                Glib::ustring::sprintf(
                                    _("Begin request to %s.."),
                                    pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                               : pageNavigation->get_request_path()
                                ),
                                .5
                            );

                            // Response
                            GioSocketConnection->get_input_stream()->read_async( // | read_all_async
                                buffer,
                                sizeof(buffer) - 1,
                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                {
                                    update(
                                        MIME::UNDEFINED,
                                        _("Reading"),
                                        Glib::ustring::sprintf(
                                            _("Reading response from %s.."),
                                            pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                                       : pageNavigation->get_request_path()
                                        ),
                                        .75
                                    );

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
                                            update(
                                                MIME::TEXT_GEMINI,
                                                pageNavigation->get_request_host(), // @TODO title
                                                pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                                           : pageNavigation->get_request_path()
                                                ,
                                                1
                                            );

                                            pageContent->set_text_gemini(
                                                buffer // @TODO
                                            );
                                        }

                                        else
                                        {
                                            update(
                                                MIME::UNDEFINED,
                                                _("Oops"),
                                                _("MIME type not supported"),
                                                1
                                            );

                                            pageContent->set_text_plain( // @TODO
                                                description
                                            );
                                        }
                                    }

                                    else
                                    {
                                        update(
                                            MIME::UNDEFINED,
                                            _("Oops"),
                                            Glib::ustring::sprintf(
                                                _("Response code %s not supported"),
                                                meta[1]
                                            ),
                                            1
                                        );

                                        pageContent->set_text_plain( // @TODO
                                            description
                                        );
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

        navigation_update(
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

        navigation_update(
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

        navigation_update(
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

Glib::ustring Page::get_navigation_request_text()
{
    return pageNavigation->get_request_text();
}

// Setters
void Page::set_navbar_request_text(
    const Glib::ustring & VALUE
) {
    pageNavigation->set_request_text(
        VALUE
    );
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
                `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, `app_browser_main_tab__session_id` INTEGER NOT NULL,
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
    const int & DB__APP_BROWSER_MAIN_TAB__SESSION_ID
) {
    char * error; // @TODO
    sqlite3_stmt * statement;

    const int PREPARE_STATUS = sqlite3_prepare_v3(
        db,
        Glib::ustring::sprintf(
            R"SQL(
                SELECT * FROM `app_browser_main_tab_page__session` WHERE `app_browser_main_tab__session_id` = %d
            )SQL",
            DB__APP_BROWSER_MAIN_TAB__SESSION_ID
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
            const int APP_BROWSER_MAIN_TAB_PAGE__SESSION_ID = sqlite3_column_int(
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
                    APP_BROWSER_MAIN_TAB_PAGE__SESSION_ID
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
                    APP_BROWSER_MAIN_TAB_PAGE__SESSION_ID
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
    const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB__SESSION_ID,
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
                    `app_browser_main_tab__session_id`,
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
            DB__APP_BROWSER_MAIN_TAB__SESSION_ID,
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