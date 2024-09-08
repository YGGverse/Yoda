#include "back.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Back::Back(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__BACK
) {
    set_action_name(
        "win.main_tab_page_navigation_history_back" // @TODO
    );

    set_icon_name(
        "go-previous-symbolic"
    );

    set_tooltip_text(
        _("Back")
    );
}

void Back::refresh(
    const bool & ENABLED
) {
    // @TODO update action status
}