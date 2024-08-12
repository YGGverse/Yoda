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

        // Build tab close submenu model
        tab_close = Gio::Menu::create();

            tab_close->append(
                _("Active tab"),
                "win.tab_close"
            );

            tab_close->append(
                _("Tabs to left"),
                "win.tab_close_left"
            );

            tab_close->append(
                _("Tabs to right"),
                "win.tab_close_right"
            );

            tab_close->append(
                _("All tabs"),
                "win.tab_close_all"
            );

        tab->append_submenu(
            _("Close"),
            tab_close
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