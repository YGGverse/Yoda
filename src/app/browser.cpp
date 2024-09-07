#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    //const Glib::RefPtr<Gtk::Application> & app,
    //const std::shared_ptr<lib::Database> & db
) {
    // Init window actions
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

    add_action(
        "debug",
        [this]
        {
            // @TODO https://gitlab.gnome.org/GNOME/gtkmm/-/commit/5f3b82537d3daad7bda59dd01e719788070f4b6c
            gtk_window_set_interactive_debugging(
                true
            );
        }
    );

        // Tab actions
        add_action(
            "main_tab_append",
            [this]
            {
                browserMain->tab_append();
            }
        );

        add_action(
            "main_tab_close",
            [this]
            {
                browserMain->tab_close();
            }
        )->set_enabled(
            false
        );

        add_action(
            "main_tab_close_left",
            [this]
            {
                browserMain->tab_close_left();
            }
        )->set_enabled(
            false
        );

        add_action(
            "main_tab_close_right",
            [this]
            {
                browserMain->tab_close_right();
            }
        )->set_enabled(
            false
        );

        add_action(
            "main_tab_close_all",
            [this]
            {
                browserMain->tab_close_all();
            }
        )->set_enabled(
            false
        );

            // Tab page navigation actions
            add_action(
                "main_tab_page_navigation_update",
                [this]
                {
                    browserMain->tab_page_navigation_update();
                }
            )->set_enabled(
                false
            );

            add_action(
                "main_tab_page_navigation_history_back",
                [this]
                {
                    browserMain->tab_page_navigation_history_back();
                }
            )->set_enabled(
                false
            );

            add_action(
                "main_tab_page_navigation_history_forward",
                [this]
                {
                    browserMain->tab_page_navigation_history_forward();
                }
            )->set_enabled(
                false
            );

    // Init widget
    set_title(
        _("Yoda")
    );

    set_default_size(
        WIDTH,
        HEIGHT
    );

    // Init header widget
    browserHeader = Gtk::make_managed<browser::Header>();

    set_titlebar(
        * browserHeader
    );

    // Init main widget
    browserMain = Gtk::make_managed<browser::Main>();

    set_child(
        * browserMain
    );
}