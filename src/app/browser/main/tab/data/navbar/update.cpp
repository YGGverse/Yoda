#include "update.hpp"

using namespace app::browser::main::tab::data::navbar;

Update::Update()
{
    set_action_name(
        "data.update"
    );

    set_icon_name(
        "view-refresh-symbolic"
    );

    set_tooltip_text(
        _("Update")
    );

    set_sensitive(
        false
    );
}

Update::~Update() = default;
