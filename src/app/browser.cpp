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
        WIDTH,
        HEIGHT
    );

    // Init header widget
    browserHeader = new browser::Header();

    set_titlebar(
        * browserHeader
    );

    // Init main widget
    browserMain = new browser::Main();

    set_child(
        * browserMain
    );

    // Init actions
    add_action(
        "tab_append",
        [this]
        {
            browserMain->tab_append();
        }
    );

    add_action(
        "tab_update",
        [this]
        {
            browserMain->tab_update();
        }
    );

    // Close
    add_action(
        "tab_close",
        [this]
        {
            browserMain->tab_close();
        }
    );

        // Close submenu
        add_action(
            "tab_close_left",
            [this]
            {
                browserMain->tab_close_left();
            }
        );

        add_action(
            "tab_close_right",
            [this]
            {
                browserMain->tab_close_right();
            }
        );

        add_action(
            "tab_close_all",
            [this]
            {
                browserMain->tab_close_all();
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

    // Hidden
    add_action(
        "refresh",
        [this]
        {
            browserMain->refresh();

            browserHeader->set_title(
                browserMain->get_current_tab_page_title()
            );

            browserHeader->set_subtitle(
                browserMain->get_current_tab_page_subtitle()
            );
        }
    );
}

Browser::~Browser()
{
    delete browserHeader;
    delete browserMain;
}