#include "forward.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Forward::Forward(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__FORWARD
) {
    // Init actions
    action__forward = ACTION__FORWARD;

    // Init widget
    set_icon_name(
        "go-next-symbolic"
    );

    set_tooltip_text(
        _("Forward")
    );

    set_sensitive(
        false
    );

    signal_clicked().connect(
        [this]
        {
            action__forward->activate();
        }
    );
}

void Forward::refresh(
    const bool & ENABLED
) {
    set_sensitive(
        ENABLED
    );

    action__forward->set_enabled(
        ENABLED
    );
}