#include "page.hpp"
#include "page/content.hpp"
#include "page/navigation.hpp"
#include "page/progress.hpp"

using namespace app::browser::main::tab;

Page::Page(
    const Glib::ustring & TITLE,
    const Glib::ustring & SUBTITLE,
    const Glib::ustring & REQUEST
) {
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init actions group
    auto GioSimpleActionGroup = Gio::SimpleActionGroup::create();

        // Define group actions
        GioSimpleActionGroup->add_action(
            "update",
            [this]
            {
                Page::update();
            }
        );

    insert_action_group(
        "page",
        GioSimpleActionGroup
    );

    // Init components
    pageNavigation = Gtk::make_managed<page::Navigation>(
        REQUEST
    );

        append(
            * pageNavigation
        );

    pageProgress = Gtk::make_managed<page::Progress>();

        append(
            * pageProgress
        );

    pageContent = Gtk::make_managed<page::Content>();

        append(
            * pageContent
        );

    // Init extras
    refresh(
        TITLE,
        SUBTITLE,
        0
    );
}

// Getters
Glib::ustring Page::get_title()
{
    return title;
}

Glib::ustring Page::get_subtitle()
{
    return subtitle;
}

// Actions
bool Page::navigation_history_try_back()
{
    return pageNavigation->history_try_back();
}

bool Page::navigation_history_try_forward()
{
    return pageNavigation->history_try_forward();
}

void Page::refresh(
    const Glib::ustring & TITLE,
    const Glib::ustring & SUBTITLE,
    const double & PROGRESS
) {
    title = TITLE;

    subtitle = SUBTITLE;

    pageProgress->refresh(
        PROGRESS
    );

    activate_action(
        "win.refresh"
    );
}

void Page::update(
    const bool & HISTORY
) {
    // Update navigation history
    if (HISTORY)
    {
        pageNavigation->history_add(
            pageNavigation->get_request_text()
        );
    }

    // Update page extras
    refresh(
        pageNavigation->get_request_host(),
        Glib::ustring::sprintf(
            _("load %s.."),
            pageNavigation->get_request_text()
        ), 0
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
                refresh(
                    pageNavigation->get_request_host(),
                    Glib::ustring::sprintf(
                        _("connect %s.."),
                        pageNavigation->get_request_host()
                    ), .25
                );

                try
                {
                    GioSocketConnection = GioSocketClient->connect_to_uri_finish(
                        result
                    );
                }

                catch (const Glib::Error & EXCEPTION)
                {
                    refresh(
                        pageNavigation->get_request_host(),
                        EXCEPTION.what(), 1
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
                            refresh(
                                pageNavigation->get_request_host(),
                                Glib::ustring::sprintf(
                                    _("request %s.."),
                                    pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                           : pageNavigation->get_request_path()
                                ), .5
                            );

                            // Response
                            GioSocketConnection->get_input_stream()->read_async( // | read_all_async
                                buffer,
                                sizeof(buffer) - 1,
                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                {
                                    refresh(
                                        pageNavigation->get_request_host(),
                                        Glib::ustring::sprintf(
                                            _("reading %s.."),
                                            pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                                   : pageNavigation->get_request_path()
                                        ), .75
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
                                            pageContent->set_text_gemini(
                                                buffer // @TODO
                                            );
                                        }

                                        else
                                        {
                                            pageContent->set_text_plain(
                                                _("MIME type not supported")
                                            );
                                        }
                                    }

                                    else
                                    {
                                        pageContent->set_text_plain(
                                            _("Could not open page")
                                        );
                                    }

                                    GioSocketConnection->close();

                                    refresh(
                                        pageNavigation->get_request_host(), // @TODO title
                                        pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                               : pageNavigation->get_request_path()
                                        , 1
                                    );
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

        update();
    }

    else
    {
        // @TODO search request
    }
}