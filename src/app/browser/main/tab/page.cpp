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
    // Init extras
    title = TITLE;
    subtitle = SUBTITLE;

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

        // Refresh children elements view (e.g. buttons sensitivity)
        // because of insert_action_group + append here @TODO
        pageNavbar->refresh();

    pageProgressbar = new page::Progressbar();

        append(
            * pageProgressbar
        );

    pageContent = new page::Content();

        append(
            * pageContent
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

// Actions
void Page::update()
{
    title = _("Loading..");

    // Reset progress
    pageProgressbar->set(
        0
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

        GioSocketClient_RefPtr->connect_to_host_async(
            pageNavbar->get_request_host(),
            pageNavbar->get_request_port().empty() ? 1965 : std::stoi(
                pageNavbar->get_request_port()
            ),
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                pageProgressbar->set(
                    .25
                );

                GioSocketConnection_RefPtr = GioSocketClient_RefPtr->connect_to_host_finish(
                    result
                );

                // Request
                const Glib::ustring navbar_request_text = pageNavbar->get_request_text() + "\r\n";

                GioSocketConnection_RefPtr->get_output_stream()->write_async(
                    navbar_request_text.data(),
                    navbar_request_text.size(),
                    [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                    {
                        pageProgressbar->set(
                            .5
                        );

                        // Response
                        GioSocketConnection_RefPtr->get_input_stream()->read_async( // | read_all_async
                            buffer,
                            sizeof(buffer) - 1,
                            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                            {
                                pageProgressbar->set(
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

                                pageProgressbar->set(
                                    1
                                );
                            }
                        );
                    }
                );
            }
        );
    }

    // Scheme not found but host provided, redirect to gemini://
    else if (!pageNavbar->get_request_host().empty())
    {
        Glib::ustring navbar_request_text = "gemini://";

        navbar_request_text += pageNavbar->get_request_host(); // @TODO validate

        if (!pageNavbar->get_request_port().empty())
        {
            navbar_request_text += pageNavbar->get_request_port();
        }

        navbar_request_text += pageNavbar->get_request_path();

        if (!pageNavbar->get_request_query().empty())
        {
            navbar_request_text += "?" + pageNavbar->get_request_query();
        }

        pageNavbar->set_request_text(
            navbar_request_text
        );

        update();
    }

    else
    {
        // @TODO search request
    }
}