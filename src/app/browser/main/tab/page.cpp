#include "page.hpp"
#include "page/navbar.hpp"
#include "page/content.hpp"

using namespace app::browser::main::tab;

Page::Page()
{
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
    navbar = new page::Navbar();

        append(
            * navbar
        );

        // Refresh children elements view (e.g. buttons sensitivity)
        // because of insert_action_group + append here @TODO
        navbar->refresh();

    content = new page::Content();

        append(
            * content
        );
}

Page::~Page()
{
    delete navbar;
    delete content;
}

void Page::update()
{
    // Route by request scheme
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
            navbar->get_request_port().empty() ? 1965 : stoi(
                navbar->get_request_port()
            ),
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                socket_connection = socket_client->connect_to_host_finish(
                    result
                );

                // Request
                const std::string request = navbar->get_request() + "\r\n";

                socket_connection->get_output_stream()->write_async(
                    request.data(),
                    request.size(),
                    [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                    {
                        // Response
                        socket_connection->get_input_stream()->read_all_async( // | read_async
                            buffer,
                            sizeof(buffer) - 1,
                            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                            {
                                content->set(
                                    buffer
                                );

                                socket_connection->close();
                            }
                        );
                    }
                );
            }
        );
    }

    else
    {
        // @TODO
    }
}