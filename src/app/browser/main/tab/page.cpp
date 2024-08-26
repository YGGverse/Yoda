#include "page.hpp"
#include "page/content.hpp"
#include "page/navbar.hpp"
#include "page/progressbar.hpp"

using namespace app::browser::main::tab;

Page::Page(
    const Glib::ustring & navbar_request_text
) {
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init actions group
    action_group = Gio::SimpleActionGroup::create();

        // Define group actions
        action_group->add_action(
            "update",
            [this]
            {
                Page::update();
            }
        );

    insert_action_group(
        "page",
        action_group
    );

    // Init components
    navbar = new page::Navbar(
        navbar_request_text
    );

        append(
            * navbar
        );

        // Refresh children elements view (e.g. buttons sensitivity)
        // because of insert_action_group + append here @TODO
        navbar->refresh();

    progressbar = new page::Progressbar();

        append(
            * progressbar
        );

    content = new page::Content();

        append(
            * content
        );
}

Page::~Page()
{
    delete navbar;
    delete content;
    delete progressbar;
}

void Page::update()
{
    // Reset progress
    progressbar->set(
        0
    );

    // Connect scheme driver
    if ("file" == navbar->get_request_scheme())
    {
        // @TODO
    }

    else if ("gemini" == navbar->get_request_scheme())
    {
        // Create new socket connection
        socket_client = Gio::SocketClient::create();

        socket_client->set_tls(
            true
        );

        socket_client->set_tls_validation_flags(
            Gio::TlsCertificateFlags::NO_FLAGS
        );

        socket_client->set_timeout(
            15 // @TODO
        );

        socket_client->connect_to_host_async(
            navbar->get_request_host(),
            navbar->get_request_port().empty() ? 1965 : std::stoi(
                navbar->get_request_port()
            ),
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                progressbar->set(
                    .25
                );

                socket_connection = socket_client->connect_to_host_finish(
                    result
                );

                // Request
                const Glib::ustring navbar_request_text = navbar->get_request_text() + "\r\n";

                socket_connection->get_output_stream()->write_async(
                    navbar_request_text.data(),
                    navbar_request_text.size(),
                    [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                    {
                        progressbar->set(
                            .5
                        );

                        // Response
                        socket_connection->get_input_stream()->read_async( // | read_all_async
                            buffer,
                            sizeof(buffer) - 1,
                            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                            {
                                progressbar->set(
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
                                    if (meta[2] == "text/gemini" || Glib::str_has_suffix(navbar->get_request_path(), ".gmi"))
                                    {
                                        content->set_text_gemini(
                                            buffer // @TODO
                                        );
                                    }

                                    else
                                    {
                                        content->set_text_plain(
                                            _("MIME type not supported")
                                        );
                                    }
                                }

                                else
                                {
                                    content->set_text_plain(
                                        _("Could not open page")
                                    );
                                }

                                socket_connection->close();

                                progressbar->set(
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
    else if (!navbar->get_request_host().empty())
    {
        Glib::ustring navbar_request_text = "gemini://";

        navbar_request_text += navbar->get_request_host(); // @TODO validate

        if (!navbar->get_request_port().empty())
        {
            navbar_request_text += navbar->get_request_port();
        }

        navbar_request_text += navbar->get_request_path();

        if (!navbar->get_request_query().empty())
        {
            navbar_request_text += "?" + navbar->get_request_query();
        }

        navbar->set_request_text(
            navbar_request_text
        );

        update();
    }

    else
    {
        // @TODO search request
    }
}