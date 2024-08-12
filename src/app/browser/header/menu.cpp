#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    // Set widget defaults
    set_tooltip_text(
        TOOLTIP
    );

    // Build tab submenu model
    tab = Gio::Menu::create();

        tab->append(
            _("New tab.."),
            "win.tab_append"
        );

        tab->append(
            _("Close active"),
            "win.tab_close"
        );

        tab->append(
            _("Close all"),
            "win.tab_close_all"
        );

    // Build tool submenu model
    tool = Gio::Menu::create();

        tool->append(
            _("Debug"),
            "win.debug"
        );

    // Build main menu model
    main = Gio::Menu::create();

        main->append_submenu(
            _("Tab"),
            tab
        );

        main->append_submenu(
            _("Tool"),
            tool
        );

        main->append(
            _("Quit"),
            "app.quit"
        );

    // Apply model
    set_menu_model(
        main
    );
}

Menu::~Menu() = default;