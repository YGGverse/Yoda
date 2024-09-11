#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    sqlite3 * db,
    const Glib::RefPtr<Gtk::Application> & APP
) {
    // Init window actions
    const auto ACTION__REFRESH = add_action(
        "refresh",
        [this]
        {
            browserMain->update();

            browserHeader->update(
                browserMain->get_current_tab_page_title(),
                browserMain->get_current_tab_page_description()
            );
        }
    );

    const auto ACTION__RESTORE = add_action(
        "restore",
        [this]
        {
            browserMain->restore();
        }
    );

    const auto ACTION__SAVE = add_action(
        "save",
        [this]
        {
            browserMain->save();
        }
    );

    const auto ACTION__DEBUG = add_action(
        "debug",
        [this]
        {
            // @TODO https://gitlab.gnome.org/GNOME/gtkmm/-/commit/5f3b82537d3daad7bda59dd01e719788070f4b6c
            gtk_window_set_interactive_debugging(
                true
            );
        }
    );

        APP->set_accel_for_action(
            "win.debug",
            "<Primary>i"
        );

    const auto ACTION__MAIN_TAB_APPEND = add_action(
        "main_tab_append",
        [this]
        {
            browserMain->tab_append();
        }
    );

        APP->set_accel_for_action(
            "win.main_tab_append",
            "<Primary>t"
        );

    const auto ACTION__MAIN_TAB_CLOSE_ACTIVE = add_action(
        "main_tab_close_active",
        [this]
        {
            browserMain->tab_close();
        }
    );

        ACTION__MAIN_TAB_CLOSE_ACTIVE->set_enabled(
            false
        );

        APP->set_accel_for_action(
            "win.main_tab_close_active",
            "<Primary>Escape"
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

    const auto ACTION__MAIN_TAB_CLOSE_ALL = add_action(
        "main_tab_close_all",
        [this]
        {
            browserMain->tab_close_all();
        }
    );

        ACTION__MAIN_TAB_CLOSE_ALL->set_enabled(
            false
        );

    const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD = add_action(
        "main_tab_page_navigation_reload",
        [this]
        {
            browserMain->tab_page_navigation_reload();
        }
    );

        ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD->set_enabled(
            false
        );

        APP->set_accel_for_action(
            "win.main_tab_page_navigation_reload",
            "<Primary>r"
        );

    const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK = add_action(
        "main_tab_page_navigation_history_back",
        [this]
        {
            browserMain->tab_page_navigation_history_back();
        }
    );

        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK->set_enabled(
            false
        );

        APP->set_accel_for_action(
            "win.main_tab_page_navigation_history_back",
            "<Primary>Left"
        );

    const auto ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD = add_action(
        "main_tab_page_navigation_history_forward",
        [this]
        {
            browserMain->tab_page_navigation_history_forward();
        }
    );

        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD->set_enabled(
            false
        );

        APP->set_accel_for_action(
            "win.main_tab_page_navigation_history_forward",
            "<Primary>Right"
        );

    const auto ACTION__QUIT = add_action(
        "quit",
        [this]
        {
            close();
        }
    );

        APP->set_accel_for_action(
            "win.quit",
            "<Primary>q"
        );

    // Init widget
    set_title(
        _("Yoda")
    );

    set_default_size(
        WIDTH,
        HEIGHT
    );

    // Init components
    browserHeader = Gtk::make_managed<browser::Header>(
        ACTION__DEBUG,
        ACTION__QUIT,
        ACTION__RESTORE,
        ACTION__SAVE,
        ACTION__MAIN_TAB_APPEND,
        ACTION__MAIN_TAB_CLOSE_ACTIVE,
        ACTION__MAIN_TAB_CLOSE_ALL,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
    );

        set_titlebar(
            * browserHeader
        );

    browserMain = Gtk::make_managed<browser::Main>(
        db,
        ACTION__REFRESH,
        ACTION__MAIN_TAB_CLOSE_ACTIVE,
        ACTION__MAIN_TAB_CLOSE_ALL,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
    );

        set_child(
            * browserMain
        );

    // Connect signals
    signal_close_request().connect(
        [this]
        {
            browserMain->save();

            // @TODO sqlite3_close(db);

            return false;
        },
        true
    );
}