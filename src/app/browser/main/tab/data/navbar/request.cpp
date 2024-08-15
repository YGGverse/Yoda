#include "request.hpp"

using namespace app::browser::main::tab::data::navbar;
using namespace std;

Request::Request()
{
    // Init entry
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        true
    );

    parse();

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
                "data.update"
            );
        }
    );
}

// Getters
string Request::get_scheme()
{
    return scheme;
}

string Request::get_host()
{
    return host;
}

string Request::get_path()
{
    return path;
}

string Request::get_query()
{
    return path;
}

int Request::get_port()
{
    return stoi(
        port
    );
}

// Private helpers
void Request::parse() // make private??
{
    string subject = get_text();

    smatch results;

    static const regex pattern( // @TODO user:password@#fragment?
        R"regex(^((\w+)?:\/\/)?([^:\/]+)?(:(\d+)?)?([^\?$]+)?(\?(.*)?)?)regex"
    );

    regex_search(
        subject,
        results,
        pattern
    );

    scheme = results[2];
    host   = results[3];
    port   = results[5];
    path   = results[6];
    query  = results[8];
}

Request::~Request() = default;