#include "tab.hpp"

using namespace app::browser::header;

Tab::Tab(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_APPEND
) {
    // Init actions
    action__tab_append = ACTION__TAB_APPEND;

    // Init widget
    set_icon_name(
        "tab-new-symbolic"
    );

    set_tooltip_text(
        _("New tab")
    );

    // Init events
    signal_clicked().connect(
        [this]
        {
            action__tab_append->activate();
        }
    );
}