#include "reload.hpp"

using namespace app::browser::main::tab::page::navigation;

Reload::Reload(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD
) {
    // Init actions
    action__reload = ACTION__RELOAD;

    // Init widget
    set_icon_name(
        "view-refresh-symbolic"
    );

    set_tooltip_text(
        _("Reload")
    );

    set_sensitive(
        false
    );

    signal_clicked().connect(
        [this]
        {
            action__reload->activate();
        }
    );
}

void Reload::update(
    const bool & IS_ENABLED
) {
    set_sensitive(
        IS_ENABLED
    );

    action__reload->set_enabled(
        IS_ENABLED
    );
}