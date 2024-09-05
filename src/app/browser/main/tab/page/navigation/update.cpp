#include "update.hpp"

using namespace app::browser::main::tab::page::navigation;

Update::Update()
{
    set_action_name(
        "win.main_tab_page_navigation_update"
    );

    set_icon_name(
        "view-refresh-symbolic"
    );

    set_tooltip_text(
        _("Update")
    );
}