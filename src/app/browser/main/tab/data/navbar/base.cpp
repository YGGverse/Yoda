#include "base.hpp"

using namespace app::browser::main::tab::data::navbar;

Base::Base()
{
    set_action_name(
        "tab.base"
    );

    set_icon_name(
        "go-home-symbolic"
    );

    set_tooltip_text(
        _("Base")
    );
}

Base::~Base() = default;
