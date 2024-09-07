#include "forward.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Forward::Forward()
{
    set_action_name(
        "win.main_tab_page_navigation_history_forward"
    );

    set_icon_name(
        "go-next-symbolic"
    );

    set_tooltip_text(
        _("Forward")
    );
}