#include "request.hpp"

using namespace app::browser::main::tab::page::navigation;

// Construct
Request::Request(
    const Glib::ustring & TEXT
) {
    // Init entry
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        HEXPAND
    );

    if (!TEXT.empty())
    {
        set_text(
            TEXT
        );

        parse();
    }

    // Connect events
    signal_changed().connect(
        [this]
        {
            parse();

            activate_action(
                "navigation.refresh"
            );
        }
    );

    signal_activate().connect(
        [this]
        {
            parse();

            activate_action(
                "win.main_tab_page_navigation_update"
            );
        }
    );
}

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

    for (const Glib::ustring & VALUE : match)
    {
        switch (index)
        {
            case 2: scheme = VALUE; break;
            case 3: host   = VALUE; break;
            case 5: port   = VALUE; break;
            case 6: path   = VALUE; break;
            case 8: query  = VALUE; break;
        }

        index++;
    }
}