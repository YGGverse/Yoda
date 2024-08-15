#include "request.hpp"

using namespace app::browser::main::tab::data::navbar;

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
            activate_action(
                "navbar.refresh"
            );
        }
    );

    signal_activate().connect(
        [this]
        {
            activate_action(
                "data.update"
            );
        }
    );
}

Request::~Request() = default;