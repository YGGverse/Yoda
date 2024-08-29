#include "forward.hpp"

using namespace app::browser::main::tab::page::navbar::history;

Forward::Forward()
{
    set_action_name(
        "win.tab_history_forward"
    );

    set_icon_name(
        "go-next-symbolic"
    );

    set_tooltip_text(
        _("Forward")
    );

    set_sensitive(
        false // @TODO no effect by set_action_name
    );
}

Forward::~Forward() = default;
