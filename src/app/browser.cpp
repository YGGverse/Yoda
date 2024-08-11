#include "browser.hpp"
#include "browser/header.hpp"
#include "browser/main.hpp"

using namespace app;

Browser::Browser(
    const Glib::RefPtr<Gtk::Application> & app,
    const lib::Database & db
) {
    // Init window
    set_title(
        TITLE
    );

    set_default_size(
        WIDTH,
        HEIGHT
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
        "tab",
        sigc::mem_fun(
            * this,
            & Browser::mainTabAppend
        )
    );

    add_action(
        "debug",
        sigc::mem_fun(
            * this,
            & Browser::debug
        )
    );

    // Init
    app->set_accel_for_action(
        "win.tab",
        "<Primary>t"
    );

    app->set_accel_for_action(
        "win.debug",
        "<Primary>i"
    );
}

void Browser::mainTabAppend()
{
    main->tabAppend();
};

void Browser::debug()
{
    gtk_window_set_interactive_debugging(
        true
    );
};