#include "base.hpp"

using namespace app::browser::main::tab::page::navigation;

Base::Base()
{
    set_icon_name(
        "go-home-symbolic"
    );

    set_tooltip_text(
        _("Base")
    );

    set_sensitive(
        false
    );
}