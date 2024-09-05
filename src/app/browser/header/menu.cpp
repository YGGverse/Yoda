#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu()
{
    set_tooltip_text(
        _("Menu")
    );

    set_menu_model(
        main(
            main_tab(
                main_tab_page(
                    main_tab_page_navigation(
                        main_tab_page_navigation_history()
                    )
                ),
                main_tab_close()
            ),
            main_tools()
        )
    );
}

Glib::RefPtr<Gio::Menu> Menu::main(
    const Glib::RefPtr<Gio::Menu> & MAIN_TAB,
    const Glib::RefPtr<Gio::Menu> & MAIN_TOOLS
) {
    auto menu = Gio::Menu::create();

    menu->append_submenu(
        _("Tab"),
        MAIN_TAB
    );

    menu->append_submenu(
        _("Tools"),
        MAIN_TOOLS
    );

    menu->append(
        _("Quit"),
        "app.quit"
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab(
    const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE,
    const Glib::RefPtr<Gio::Menu> & MAIN_TAB_CLOSE
) {
    auto menu = Gio::Menu::create();

    menu->append(
        _("New.."),
        "win.main_tab_append"
    );

    menu->append_submenu(
        _("Page"),
        MAIN_TAB_PAGE
    );

    menu->append_submenu(
        _("Close"),
        MAIN_TAB_CLOSE
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page(
    const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE_NAVIGATION
) {
    auto menu = Gio::Menu::create();

    menu->append_submenu(
        _("Navigation"),
        MAIN_TAB_PAGE_NAVIGATION
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page_navigation(
    const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE_NAVIGATION_HISTORY
) {
    auto menu = Gio::Menu::create();

    menu->append_submenu(
        _("History"),
        MAIN_TAB_PAGE_NAVIGATION_HISTORY
    );

    menu->append(
        _("Update"),
        "win.main_tab_page_navigation_update"
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page_navigation_history()
{
    auto menu = Gio::Menu::create();

    menu->append(
        _("Back"),
        "win.main_tab_page_navigation_history_try_back"
    );

    menu->append(
        _("Forward"),
        "win.main_tab_page_navigation_history_try_forward"
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_close()
{
    auto menu = Gio::Menu::create();

    menu->append(
        _("Active tab"),
        "win.main_tab_close"
    );

    menu->append(
        _("All tabs to left"),
        "win.main_tab_close_left"
    );

    menu->append(
        _("All tabs to right"),
        "win.main_tab_close_right"
    );

    menu->append(
        _("All tabs"),
        "win.main_tab_close_all"
    );

    return menu;
}

Glib::RefPtr<Gio::Menu> Menu::main_tools()
{
    auto menu = Gio::Menu::create();

    menu->append(
        _("Debug"),
        "win.debug"
    );

    return menu;
}