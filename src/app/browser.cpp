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
    browserHeader = Gtk::make_managed<browser::Header>();

    set_titlebar(
        * browserHeader
    );

    // Init main widget
    browserMain = Gtk::make_managed<browser::Main>();

    set_child(
        * browserMain
    );

    // Init browser window actions
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
        );

        add_action(
            "main_tab_close_left",
            [this]
            {
                browserMain->tab_close_left();
            }
        );

        add_action(
            "main_tab_close_right",
            [this]
            {
                browserMain->tab_close_right();
            }
        );

        add_action(
            "main_tab_close_all",
            [this]
            {
                browserMain->tab_close_all();
            }
        );

            // Tab page navigation actions
            add_action(
                "main_tab_page_navigation_update",
                [this](const bool & ADD_HISTORY = false)
                {
                    browserMain->tab_page_navigation_update(
                        ADD_HISTORY
                    );
                }
            );

            add_action(
                "main_tab_page_navigation_history_try_back",
                [this]()
                {
                    browserMain->tab_page_navigation_history_try_back();
                }
            );

            add_action(
                "main_tab_page_navigation_history_try_forward",
                [this]
                {
                    browserMain->tab_page_navigation_history_try_forward();
                }
            );
}