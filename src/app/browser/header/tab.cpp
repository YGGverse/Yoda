#include "tab.hpp"

using namespace app::browser::header;

Tab::Tab()
{
    set_tooltip_text(
        TOOLTIP
    );

    set_icon_name(
        ICON
    );

    set_action_name(
        "win.tab"
    );
}

Tab::~Tab() = default;