#include "bookmark.hpp"

using namespace app::browser::main::tab::page::navigation;

Bookmark::Bookmark()
{
    set_action_name(
        "tab.bookmark"
    );

    set_icon_name(
        "starred-symbolic" // | non-starred-symbolic
    );

    set_tooltip_text(
        _("Toggle bookmark")
    );
}