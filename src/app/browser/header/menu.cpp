#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    // Set widget defaults
    set_tooltip_text(
        _("Menu")
    );

    // Build tab submenu model
    tab = Gio::Menu::create();

        tab->append(
            _("New tab.."),
            "win.main_tab_append"
        );

        // Build tab page submenu model
        tab_page = Gio::Menu::create();

            tab_page->append(
                _("Update"),
                "win.main_tab_page_update"
            );

        tab->append_submenu(
            _("Page"),
            tab_page
        );

        // Build tab close submenu model
        tab_close = Gio::Menu::create();

            tab_close->append(
                _("Active tab"),
                "win.main_tab_close"
            );

            tab_close->append(
                _("All tabs to left"),
                "win.main_tab_close_left"
            );

            tab_close->append(
                _("All tabs to right"),
                "win.main_tab_close_right"
            );

            tab_close->append(
                _("All tabs"),
                "win.main_tab_close_all"
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
            _("Tools"),
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