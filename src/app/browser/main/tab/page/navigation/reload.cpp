#include "reload.hpp"

using namespace app::browser::main::tab::page::navigation;

Reload::Reload(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init actions
    action__tab_page_navigation_reload = ACTION__TAB_PAGE_NAVIGATION_RELOAD;

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
            action__tab_page_navigation_reload->activate();
        }
    );
}

void Reload::update(
    const bool & IS_ENABLED
) {
    set_sensitive(
        IS_ENABLED
    );

    action__tab_page_navigation_reload->set_enabled(
        IS_ENABLED
    );
}