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
}

Browser::~Browser()
{
    destroy();

    delete header;
    header = nullptr;

    delete main;
    main = nullptr;
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