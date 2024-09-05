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
    Glib::RefPtr<Gio::Menu> main_tab,
    Glib::RefPtr<Gio::Menu> main_tools
) {
    auto model = Gio::Menu::create();

    model->append_submenu(
        _("Tab"),
        main_tab
    );

    model->append_submenu(
        _("Tools"),
        main_tools
    );

    model->append(
        _("Quit"),
        "app.quit"
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab(
    Glib::RefPtr<Gio::Menu> main_tab_page,
    Glib::RefPtr<Gio::Menu> main_tab_close
) {
    auto model = Gio::Menu::create();

    model->append(
        _("New.."),
        "win.main_tab_append"
    );

    model->append_submenu(
        _("Page"),
        main_tab_page
    );

    model->append_submenu(
        _("Close"),
        main_tab_close
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page(
    Glib::RefPtr<Gio::Menu> main_tab_page_navigation
) {
    auto model = Gio::Menu::create();

    model->append_submenu(
        _("Navigation"),
        main_tab_page_navigation
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page_navigation(
    Glib::RefPtr<Gio::Menu> main_tab_page_navigation_history
) {
    auto model = Gio::Menu::create();

    model->append_submenu(
        _("History"),
        main_tab_page_navigation_history
    );

    model->append(
        _("Update"),
        "win.main_tab_page_update"
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_page_navigation_history()
{
    auto model = Gio::Menu::create();

    model->append(
        _("Back"),
        "win.main_tab_page_navigation_history_try_back"
    );

    model->append(
        _("Forward"),
        "win.main_tab_page_navigation_history_try_forward"
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tab_close()
{
    auto model = Gio::Menu::create();

    model->append(
        _("Active tab"),
        "win.main_tab_close"
    );

    model->append(
        _("All tabs to left"),
        "win.main_tab_close_left"
    );

    model->append(
        _("All tabs to right"),
        "win.main_tab_close_right"
    );

    model->append(
        _("All tabs"),
        "win.main_tab_close_all"
    );

    return model;
}

Glib::RefPtr<Gio::Menu> Menu::main_tools()
{
    auto model = Gio::Menu::create();

    model->append(
        _("Debug"),
        "win.debug"
    );

    return model;
}