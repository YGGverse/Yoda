#include "page.hpp"
#include "page/content.hpp"
#include "page/navbar.hpp"
#include "page/progressbar.hpp"

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
    auto GioSimpleActionGroup_RefPtr = Gio::SimpleActionGroup::create();

        // Define group actions
        GioSimpleActionGroup_RefPtr->add_action(
            "update",
            [this]
            {
                Page::update();
            }
        );

    insert_action_group(
        "page",
        GioSimpleActionGroup_RefPtr
    );

    // Init components
    pageNavbar = new page::Navbar(
        REQUEST
    );

        append(
            * pageNavbar
        );

    pageProgressbar = new page::Progressbar();

        append(
            * pageProgressbar
        );

    pageContent = new page::Content();

        append(
            * pageContent
        );

    // Init extras
    set(
        TITLE,
        SUBTITLE,
        0
    );
}

Page::~Page()
{
    delete pageNavbar;
    delete pageContent;
    delete pageProgressbar;
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
void Page::back()
{
    pageNavbar->back();
}

void Page::forward()
{
    pageNavbar->forward();
}

void Page::update()
{
    // Update navigation history
    pageNavbar->push(
        pageNavbar->get_request_text()
    );

    // Update page extras
    set(
        pageNavbar->get_request_host(),
        Glib::ustring::sprintf(
            _("load %s.."),
            pageNavbar->get_request_text()
        ), 0
    );

    // Connect scheme driver
    if ("file" == pageNavbar->get_request_scheme())
    {
        // @TODO
    }

    else if ("gemini" == pageNavbar->get_request_scheme())
    {
        // Create new socket connection
        GioSocketClient_RefPtr = Gio::SocketClient::create();

        GioSocketClient_RefPtr->set_tls(
            true
        );

        GioSocketClient_RefPtr->set_tls_validation_flags(
            Gio::TlsCertificateFlags::NO_FLAGS
        );

        GioSocketClient_RefPtr->set_timeout(
            15 // @TODO
        );

        GioSocketClient_RefPtr->connect_to_uri_async(
            pageNavbar->get_request_text(), 1965,
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                set(
                    pageNavbar->get_request_host(),
                    Glib::ustring::sprintf(
                        _("connect %s.."),
                        pageNavbar->get_request_host()
                    ), .25
                );

                try
                {
                    GioSocketConnection_RefPtr = GioSocketClient_RefPtr->connect_to_uri_finish(
                        result
                    );
                }

                catch (const Glib::Error & EXCEPTION)
                {
                    set(
                        pageNavbar->get_request_host(),
                        EXCEPTION.what(), 1
                    );
                }

                // Connection established, begin request
                if (GioSocketConnection_RefPtr != nullptr)
                {
                    const Glib::ustring request = pageNavbar->get_request_text() + "\r\n";

                    GioSocketConnection_RefPtr->get_output_stream()->write_async(
                        request.data(),
                        request.size(),
                        [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                        {
                            set(
                                pageNavbar->get_request_host(),
                                Glib::ustring::sprintf(
                                    _("request %s.."),
                                    pageNavbar->get_request_path().empty() ? pageNavbar->get_request_host()
                                                                           : pageNavbar->get_request_path()
                                ), .5
                            );

                            // Response
                            GioSocketConnection_RefPtr->get_input_stream()->read_async( // | read_all_async
                                buffer,
                                sizeof(buffer) - 1,
                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                {
                                    set(
                                        pageNavbar->get_request_host(),
                                        Glib::ustring::sprintf(
                                            _("reading %s.."),
                                            pageNavbar->get_request_path().empty() ? pageNavbar->get_request_host()
                                                                                   : pageNavbar->get_request_path()
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
                                        if (meta[2] == "text/gemini" || Glib::str_has_suffix(pageNavbar->get_request_path(), ".gmi"))
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

                                    GioSocketConnection_RefPtr->close();

                                    set(
                                        pageNavbar->get_request_host(), // @TODO title
                                        pageNavbar->get_request_path().empty() ? pageNavbar->get_request_host()
                                                                               : pageNavbar->get_request_path()
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
    else if (pageNavbar->get_request_scheme().empty() && !pageNavbar->get_request_host().empty())
    {
        pageNavbar->set_request_text(
            "gemini://" + pageNavbar->get_request_text()
        );

        update();
    }

    else
    {
        // @TODO search request
    }
}

// Private helpers
void Page::set(
    const Glib::ustring & TITLE,
    const Glib::ustring & SUBTITLE,
    const double & PROGRESS
) {
    title = TITLE;

    subtitle = SUBTITLE;

    pageProgressbar->set(
        PROGRESS
    );

    activate_action(
        "win.refresh"
    );
}