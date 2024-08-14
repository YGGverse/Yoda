#include "request.hpp"

using namespace app::browser::main::tab::data::navbar;

Request::Request()
{
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        true
    );

    signal_changed().connect(
        sigc::mem_fun(
            * this,
            & Request::on_change
        )
    );

    signal_activate().connect(
        sigc::mem_fun(
            * this,
            & Request::on_activate
        )
    );
}

Request::~Request() = default;

void Request::on_activate()
{
    activate_action(
        "data.update"
    );
}

void Request::on_change()
{
    activate_action(
        "navbar.refresh"
    );
}
