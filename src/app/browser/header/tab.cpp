#include "tab.hpp"

using namespace app::browser::header;

Tab::Tab()
{
    set_action_name(
        "win.main_tab_append"
    );

    set_icon_name(
        "tab-new-symbolic"
    );

    set_tooltip_text(
        _("New tab")
    );
}