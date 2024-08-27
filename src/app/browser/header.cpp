#include "header.hpp"
#include "header/main.hpp"
#include "header/menu.hpp"
#include "header/tab.hpp"

using namespace app::browser;

Header::Header()
{
    // Init header bar
    set_show_title_buttons(
        true
    );

    // Init menu
    menu = new header::Menu();

    pack_start(
        * menu
    );

    // Init tab
    tab = new header::Tab();

    pack_start(
        * tab
    );

    // Init main widget
    main = new header::Main();

    set_title_widget(
        * main
    );
}

Header::~Header()
{
    delete main;
    delete menu;
    delete tab;
}

void Header::set_title(
    const Glib::ustring text
) {
    main->set_title(
        text
    );
}

void Header::set_subtitle(
    const Glib::ustring text
) {
    main->set_subtitle(
        text
    );
}