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
            sigc::mem_fun(
                * this,
                & Page::update
            )
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

Page::~Page() = default;

// Public actions
void Page::update()
{
    // Route by request protocol
    if ("file" == navbar->get_request_scheme())
    {
        // @TODO
    }

    else if ("gemini" == navbar->get_request_scheme())
    {
        connect(
            navbar->get_request_host(),
            navbar->get_request_port().empty() ? 1965 : stoi(
                navbar->get_request_port()
            )
        );
    }

    else
    {
        // @TODO
    }
}

// Private helpers
void Page::connect(
    const std::string & host,
    int port
) {
    try
    {
        socket_client = Gio::SocketClient::create();

        socket_client->connect_to_host_async(
            host,
            port,
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                try
                {
                    auto socket = socket_client->connect_finish(
                        result
                    );

                    // @TODO read/write data

                    socket->close();
                }

                catch (const Glib::Error & exception)
                {
                    // @TODO exception.what();
                }
            }
        );
    }

    catch (const std::exception & exception)
    {
        // @TODO exception.what();
    }
}