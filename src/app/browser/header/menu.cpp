#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    // Set widget defaults
    set_tooltip_text(
        _("Menu")
    );

    // Build tab submenu model
    main_tab = Gio::Menu::create();

        main_tab->append(
            _("New tab.."),
            "win.main_tab_append"
        );

        // Build tab page submenu model
        main_tab_page = Gio::Menu::create();

            // Build tab page navigation submenu model
            main_tab_page_navigation = Gio::Menu::create();

                // Build tab page navigation history submenu model
                main_tab_page_navigation_history = Gio::Menu::create();

                    main_tab_page_navigation_history->append(
                        _("Back"),
                        "win.main_tab_page_navigation_history_try_back"
                    );

                    main_tab_page_navigation_history->append(
                        _("Forward"),
                        "win.main_tab_page_navigation_history_try_forward"
                    );

                main_tab_page_navigation->append_submenu(
                    _("History"),
                    main_tab_page_navigation_history
                );

                main_tab_page_navigation->append(
                    _("Update"),
                    "win.main_tab_page_update"
                );

            main_tab_page->append_submenu(
                _("Navigation"),
                main_tab_page_navigation
            );

        main_tab->append_submenu(
            _("Page"),
            main_tab_page
        );

        // Build tab close submenu model
        main_tab_close = Gio::Menu::create();

            main_tab_close->append(
                _("Active tab"),
                "win.main_tab_close"
            );

            main_tab_close->append(
                _("All tabs to left"),
                "win.main_tab_close_left"
            );

            main_tab_close->append(
                _("All tabs to right"),
                "win.main_tab_close_right"
            );

            main_tab_close->append(
                _("All tabs"),
                "win.main_tab_close_all"
            );

        main_tab->append_submenu(
            _("Close"),
            main_tab_close
        );

    // Build tools submenu model
    main_tools = Gio::Menu::create();

        main_tools->append(
            _("Debug"),
            "win.debug"
        );

    // Build main menu model
    main = Gio::Menu::create();

        main->append_submenu(
            _("Tab"),
            main_tab
        );

        main->append_submenu(
            _("Tools"),
            main_tools
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