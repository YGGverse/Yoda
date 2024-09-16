#include "page.hpp"
#include "page/content.hpp"
#include "page/navigation.hpp"

using namespace app::browser::main::tab;

Page::Page(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
) {
    // Init meta
    title = _("New page");
    mime = MIME::UNDEFINED;
    progress_fraction = 0;

    // Init database
    DB::SESSION::init(
        this->db = db
    );

    // Init shared actions
    action__update = ACTION__UPDATE;

    // Init additional local action group (for clickable content)
    const auto ACTION_GROUP__PAGE = Gio::SimpleActionGroup::create();

    const auto ACTION__OPEN_LINK_VARIANT = ACTION_GROUP__PAGE->add_action_with_parameter(
        "open_link_variant",
        Glib::VARIANT_TYPE_STRING,
        [this](const Glib::VariantBase & PARAMETER)
        {
            if (PARAMETER.is_of_type(Glib::VARIANT_TYPE_STRING))
            {
                pageNavigation->set_request_text(
                    Glib::VariantBase::cast_dynamic<Glib::Variant<Glib::ustring>>(
                        PARAMETER
                    ).get()
                );

                navigation_reload(
                    true
                );
            }
        }
    );

    // Init widget
    insert_action_group(
        "page",
        ACTION_GROUP__PAGE
    );

    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init widget components
    pageNavigation = Gtk::make_managed<page::Navigation>(
        this->db,
        ACTION__HISTORY_BACK,
        ACTION__HISTORY_FORWARD,
        ACTION__OPEN_LINK_VARIANT,
        ACTION__RELOAD,
        ACTION__UPDATE
    );

        append(
            * pageNavigation
        );

    pageContent = Gtk::make_managed<page::Content>(
        ACTION__OPEN_LINK_VARIANT
    );

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
int Page::session_restore(
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
        // Restore page data from latest database record
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
            pageNavigation->session_restore(
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

void Page::session_save(
    const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
) {
    // Delegate save action to child components
    pageNavigation->session_save(
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
    // Close previous socket connection (on active)
    Socket::Connection::close(
        socket__connection
    );

    // Update navigation history?
    if (ADD_HISTORY)
    {
        // Skip same
        Glib::ustring request;

        pageNavigation->try_history_current(
            request
        );

        if (request != pageNavigation->get_request_text())
        {
            pageNavigation->history_add(
                pageNavigation->get_request_text(),
                true
            );
        }
    }

    // Parse request string
    uri = g_uri_parse(
        pageNavigation->get_request_text().c_str(),
        G_URI_FLAGS_NONE,
        NULL // @TODO GError *
    );
        // On parse fail
        if (uri == NULL)
        {
            // Request contain host substring
            if (Glib::Regex::match_simple(
                R"regex(^[^\/\s]+\.[\w]{2,})regex",
                pageNavigation->get_request_text().c_str()
            )) {
                // Append default scheme
                pageNavigation->set_request_text(
                    Glib::ustring::sprintf(
                        "gemini://%s",
                        pageNavigation->get_request_text()
                    )
                );
            }

            // Plain text given, build search request to default provider
            else
            {
                pageNavigation->set_request_text(
                    Glib::ustring::sprintf(
                        "gemini://tlgs.one/search?%s", // @TODO settings
                        g_uri_escape_string(
                            pageNavigation->get_request_text().c_str(),
                            NULL,
                            true
                        )
                    ).c_str()
                );
            }

            // Redirect @TODO limit attempts
            navigation_reload(
                false
            );
        }

    // Reset page meta data
    mime = MIME::UNDEFINED;

    title = _("Update");

    description = Glib::ustring::sprintf(
        _("Begin update for %s.."),
        pageNavigation->get_request_text()
    );

    progress_fraction = 0;

    action__update->activate();

    // Route to protocol driver by scheme
    if (g_uri_get_scheme(uri) == Glib::ustring("file"))
    {
        // @TODO
    }

    else
    if (g_uri_get_scheme(uri) == Glib::ustring("gemini"))
    {
        // Create new socket connection
        socket__client = Page::Socket::Client::Gemini::create();

        socket__client->connect_to_uri_async(
            g_uri_to_string(
                uri
            ),
            1965, // default port @TODO
            [this](const Glib::RefPtr<Gio::AsyncResult> & RESULT)
            {
                // Update
                title = _("Connect");

                description = Glib::ustring::sprintf(
                    _("Connecting to %s.."),
                    g_uri_get_host(
                        uri
                    )
                );

                progress_fraction = .25;

                action__update->activate();

                try
                {
                    socket__connection = socket__client->connect_to_uri_finish(
                        RESULT
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
                if (socket__connection != nullptr)
                {
                    // Build gemini protocol request
                    const Glib::ustring SOCKET__REQUEST = Glib::ustring::sprintf(
                        "%s\r\n",
                        g_uri_to_string(
                            uri
                        )
                    );

                    socket__connection->get_output_stream()->write_async(
                        SOCKET__REQUEST.data(),
                        SOCKET__REQUEST.size(),
                        [this](const Glib::RefPtr<Gio::AsyncResult>&)
                        {
                            // Update
                            title = _("Request");

                            description = Glib::ustring::sprintf(
                                _("Begin request to %s.."),
                                g_uri_get_host(
                                    uri
                                )
                            );

                            progress_fraction = .5;

                            action__update->activate();

                            // Response
                            socket__connection->get_input_stream()->read_all_async( // | read_async @TODO
                                buffer,
                                sizeof(
                                    buffer
                                ) - 1, // @TODO
                                [this](const Glib::RefPtr<Gio::AsyncResult>&)
                                {
                                    // Update
                                    title = _("Reading");

                                    description = Glib::ustring::sprintf(
                                        _("Reading response from %s.."),
                                        g_uri_get_host(
                                            uri
                                        )
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
                                        if (meta[2] == "text/gemini" || Glib::str_has_suffix(g_uri_get_path(uri), ".gmi"))
                                        {
                                            // Update
                                            mime = MIME::TEXT_GEMINI;

                                            title = _("Done"); // @TODO page title

                                            description = g_uri_get_host(
                                                uri
                                            );

                                            progress_fraction = 1;

                                            // Set content driver
                                            pageContent->update(
                                                page::Content::TEXT_GEMINI,
                                                buffer,
                                                uri
                                            );

                                                // Update title on detected by document provider
                                                if (!pageContent->get_title().empty())
                                                {
                                                    title = pageContent->get_title();
                                                }

                                            action__update->activate();
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

                                    Socket::Connection::close(
                                        socket__connection
                                    );
                                }
                            ); // read_all_async
                        }
                    ); // write_async
                }
            } // connect_to_uri_async
        );
    }

    else
    {
        throw _("Exception"); // @TODO
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

// Socket tools
Glib::RefPtr<Gio::SocketClient> Page::Socket::Client::create(
    const int & TIMEOUT
) {
    const auto CLIENT = Gio::SocketClient::create();

    CLIENT->set_timeout(
        TIMEOUT
    );

    return CLIENT;
}

Glib::RefPtr<Gio::SocketClient> Page::Socket::Client::Gemini::create()
{
    const auto GEMINI_CLIENT = Page::Socket::Client::create();

    GEMINI_CLIENT->set_tls(
        true
    );

    GEMINI_CLIENT->set_tls_validation_flags(
        Gio::TlsCertificateFlags::NO_FLAGS
    );

    GEMINI_CLIENT->set_protocol(
        Gio::Socket::Protocol::TCP
    );

    return GEMINI_CLIENT;
}

bool Page::Socket::Connection::is_active(
    const Glib::RefPtr<Gio::SocketConnection> & CONNECTION
) {
    return CONNECTION != nullptr && CONNECTION->is_connected();
}

bool Page::Socket::Connection::close(
    Glib::RefPtr<Gio::SocketConnection> & connection
) {
    if (Socket::Connection::is_active(connection))
    {
        if (connection->close())
        {
            connection = nullptr;

            return true;
        }
    }

    return false;
}