#include "request.hpp"

using namespace app::browser::main::tab::page::navbar;

// Construct
Request::Request()
{
    // Init entry
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        true
    );

    // Connect events
    signal_changed().connect(
        [this]
        {
            parse();

            activate_action(
                "navbar.refresh"
            );
        }
    );

    signal_activate().connect(
        [this]
        {
            parse();

            activate_action(
                "page.update"
            );
        }
    );
}

Request::~Request() = default;

// Getters
Glib::ustring Request::get_scheme()
{
    return scheme;
}

Glib::ustring Request::get_host()
{
    return host;
}

Glib::ustring Request::get_port()
{
    return port;
}

Glib::ustring Request::get_path()
{
    return path;
}

Glib::ustring Request::get_query()
{
    return path;
}

// Private helpers
void Request::parse()
{
    auto match = Glib::Regex::split_simple(
        R"regex(^((\w+)?:\/\/)?([^:\/]+)?(:(\d+)?)?([^\?$]+)?(\?(.*)?)?)regex",
        get_text()
    );

    scheme = "";
    host   = "";
    port   = "";
    path   = "";
    query  = "";

    int index = 0;

    for (const Glib::ustring & value : match)
    {
        switch (index)
        {
            case 2: scheme = value; break;
            case 3: host   = value; break;
            case 5: port   = value; break;
            case 6: path   = value; break;
            case 8: query  = value; break;
        }

        index++;
    }
}