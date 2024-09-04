#include "back.hpp"

using namespace app::browser::main::tab::page::navigation::history;

Back::Back()
{
    set_action_name(
        "win.tab_history_back"
    );

    set_icon_name(
        "go-previous-symbolic"
    );

    set_tooltip_text(
        _("Back")
    );

    set_sensitive(
        false // @TODO no effect by set_action_name
    );

    signal_clicked().connect(
        [this]
        {
            activate_action(
                "win.tab_history_back"
            );
        }
    );
}
