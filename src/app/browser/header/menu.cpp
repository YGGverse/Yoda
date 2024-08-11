#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    // Init defaults
    set_tooltip_text(
        TOOLTIP
    );

    // Build tab submenu
    auto tab = Gio::Menu::create();

    tab->append(
        _("New tab"),
        "tab.new"
    );

    // Build tool submenu
    auto tool = Gio::Menu::create();

    tool->append(
        _("Debug"),
        "win.debug"
    );

    // Build main menu
    auto main = Gio::Menu::create();

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