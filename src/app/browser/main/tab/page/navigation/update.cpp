#include "update.hpp"

using namespace app::browser::main::tab::page::navigation;

Update::Update(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
) {
    // Init actions
    action__update = ACTION__UPDATE;

    // Init widget
    set_icon_name(
        "view-refresh-symbolic"
    );

    set_tooltip_text(
        _("Update")
    );

    set_sensitive(
        false
    );

    signal_clicked().connect(
        [this]
        {
            action__update->activate();
        }
    );
}

void Update::update(
    const bool & ENABLED
) {
    set_sensitive(
        ENABLED
    );

    action__update->set_enabled(
        ENABLED
    );
}