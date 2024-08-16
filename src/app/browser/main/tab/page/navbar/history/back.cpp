#include "back.hpp"

using namespace app::browser::main::tab::page::navbar::history;

Back::Back()
{
    set_action_name(
        "tab.back"
    );

    set_icon_name(
        "go-previous-symbolic"
    );

    set_tooltip_text(
        _("Back")
    );
}

Back::~Back() = default;