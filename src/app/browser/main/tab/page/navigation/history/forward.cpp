#include "forward.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Forward::Forward(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__FORWARD
) {
    set_action_name(
        "win.main_tab_page_navigation_history_forward" // @TODO
    );

    set_icon_name(
        "go-next-symbolic"
    );

    set_tooltip_text(
        _("Forward")
    );
}

void Forward::refresh(
    const bool & ENABLED
) {
    // @TODO update action status
}