#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    // Init defaults
    set_tooltip_text(
        TOOLTIP
    );

    // Build tab submenu
    tab = Gio::Menu::create();

    tab->append(
        _("New tab.."),
        "win.tab"
    );

    // Build tool submenu
    tool = Gio::Menu::create();

    tool->append(
        _("Debug"),
        "win.debug"
    );

    // Build main menu
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

    set_menu_model(
        main
    );
}

Menu::~Menu() = default;