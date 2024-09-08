#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    //const Glib::RefPtr<Gtk::Application> & app,
    //const std::shared_ptr<lib::Database> & db
) {
    // Init window actions
    const auto ACTION__REFRESH = add_action(
        "refresh",
        [this]
        {
            browserMain->refresh();

            browserHeader->refresh(
                browserMain->get_current_tab_page_title(),
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

        const auto ACTION__MAIN_TAB_CLOSE = add_action(
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
            const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE = add_action(
                "main_tab_page_navigation_update",
                [this]
                {
                    browserMain->tab_page_navigation_update();
                }
            );

                ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE->set_enabled(
                    false
                );

            const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK = add_action(
                "main_tab_page_navigation_history_back",
                [this]
                {
                    browserMain->tab_page_navigation_history_back();
                }
            );

            const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD = add_action(
                "main_tab_page_navigation_history_forward",
                [this]
                {
                    browserMain->tab_page_navigation_history_forward();
                }
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
    browserMain = Gtk::make_managed<browser::Main>(
        ACTION__REFRESH,
        ACTION__MAIN_TAB_CLOSE,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
    );

    set_child(
        * browserMain
    );
}