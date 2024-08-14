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
        sigc::mem_fun(
            * this,
            & Browser::main_tab_append
        )
    );

    add_action(
        "tab_update",
        sigc::mem_fun(
            *this,
            &Browser::main_tab_update
        )
    );

    // Close
    add_action(
        "tab_close",
        sigc::mem_fun(
            * this,
            & Browser::main_tab_close
        )
    );

        // Close submenu
        add_action(
            "tab_close_left",
            sigc::mem_fun(
                * this,
                & Browser::main_tab_close_left
            )
        );

        add_action(
            "tab_close_right",
            sigc::mem_fun(
                * this,
                & Browser::main_tab_close_right
            )
        );

        add_action(
            "tab_close_all",
            sigc::mem_fun(
                * this,
                & Browser::main_tab_close_all
            )
        );

    // Tool
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

void Browser::main_tab_append()
{
    main->tab_append();
};

void Browser::main_tab_update()
{
    main->tab_update();
};

void Browser::main_tab_close()
{
    main->tab_close();
};

void Browser::main_tab_close_left()
{
    main->tab_close_left();
};

void Browser::main_tab_close_right()
{
    main->tab_close_right();
};

void Browser::main_tab_close_all()
{
    main->tab_close_all();
};

void Browser::debug()
{
    gtk_window_set_interactive_debugging(
        true
    );
};