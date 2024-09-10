#include "back.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Back::Back(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__BACK
) {
    // Init actions
    action__back = ACTION__BACK;

    // Init widget
    set_icon_name(
        "go-previous-symbolic"
    );

    set_tooltip_text(
        _("Back")
    );

    set_sensitive(
        false
    );

    signal_clicked().connect(
        [this]
        {
            action__back->activate();
        }
    );
}

void Back::update(
    const bool & ENABLED
) {
    set_sensitive(
        ENABLED
    );

    action__back->set_enabled(
        ENABLED
    );
}