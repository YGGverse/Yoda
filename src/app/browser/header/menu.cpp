#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    set_tooltip_text(
        TOOLTIP
    );

    // Build tab submenu
    auto tab = Gio::Menu::create();

    tab->append(
        _("New tab"),
        "app.tab.new"
    );

    // Build tool submenu
    auto tool = Gio::Menu::create();

    tool->append(
        _("Debug"),
        "app.tool.debug"
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
        "win.quit"
    );

    set_menu_model(
        main
    );
}