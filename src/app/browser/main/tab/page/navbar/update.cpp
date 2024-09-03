#include "update.hpp"

using namespace app::browser::main::tab::page::navbar;

Update::Update()
{
    set_action_name(
        "page.update"
    );

    set_icon_name(
        "view-refresh-symbolic"
    );

    set_tooltip_text(
        _("Update")
    );
}