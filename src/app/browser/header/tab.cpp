#include "tab.hpp"

using namespace app::browser::header;

Tab::Tab()
{
    set_action_name(
        "win.tab_append"
    );

    set_icon_name(
        "tab-new-symbolic"
    );

    set_tooltip_text(
        _("New tab")
    );
}

Tab::~Tab() = default;