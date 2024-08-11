#include "browser.hpp"
#include "browser/header.hpp"

using namespace app;

Browser::Browser(
    const Glib::RefPtr<Gtk::Application> & app,
    const lib::Database & db
) {
    set_title(
        TITLE
    );

    set_default_size(
        WIDTH,
        HEIGHT
    );

    set_titlebar(
        * new browser::Header()
    );

    add_action(
        "debug",
        sigc::mem_fun(
            * this,
            & Browser::debug
        )
    );

    app->set_accel_for_action(
        "win.debug",
        "<Primary>i"
    );
}

void Browser::debug()
{
    gtk_window_set_interactive_debugging(
        true
    );
};