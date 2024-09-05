#include "back.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Back::Back()
{
    set_action_name(
        "win.main_tab_page_navigation_history_back"
    );

    set_icon_name(
        "go-previous-symbolic"
    );

    set_tooltip_text(
        _("Back")
    );

    set_sensitive(
        false // @TODO no effect by set_action_name
    );
}
