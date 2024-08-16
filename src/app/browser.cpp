#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    //const Glib::RefPtr<Gtk::Application> & app,
    //const std::shared_ptr<lib::Database> & db
) {
    // Init window
    set_title(
        _("Yoda")
    );

    set_default_size(
        640,
        480
    );

    // Init header widget
    header = new browser::Header();

    set_titlebar(
        * header
    );

    // Init main widget
    main = new browser::Main();

    set_child(
        * main
    );

    // Init actions
    add_action(
        "tab_append",
        [this]
        {
            main->tab_append();
        }
    );

    add_action(
        "tab_update",
        [this]
        {
            main->tab_update();
        }
    );

    // Close
    add_action(
        "tab_close",
        [this]
        {
            main->tab_close();
        }
    );

        // Close submenu
        add_action(
            "tab_close_left",
            [this]
            {
                main->tab_close_left();
            }
        );

        add_action(
            "tab_close_right",
            [this]
            {
                main->tab_close_right();
            }
        );

        add_action(
            "tab_close_all",
            [this]
            {
                main->tab_close_all();
            }
        );

    // Tool
    add_action(
        "debug",
        [this]
        {
            gtk_window_set_interactive_debugging(
                true
            );
        }
    );
}

Browser::~Browser()
{
    delete header;
    delete main;
}