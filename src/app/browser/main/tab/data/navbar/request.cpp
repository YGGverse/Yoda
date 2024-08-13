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
}

Request::~Request() = default;
