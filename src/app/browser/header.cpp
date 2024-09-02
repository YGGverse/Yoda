#include "header.hpp"
#include "header/main.hpp"
#include "header/menu.hpp"
#include "header/tab.hpp"

using namespace app::browser;

Header::Header()
{
    // Init header bar
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );

    // Init menu
    headerMenu = Gtk::make_managed<header::Menu>();

    pack_start(
        * headerMenu
    );

    // Init tab
    headerTab = Gtk::make_managed<header::Tab>();

    pack_start(
        * headerTab
    );

    // Init main widget
    headerMain = Gtk::make_managed<header::Main>();

    set_title_widget(
        * headerMain
    );
}

void Header::set_title(
    const Glib::ustring & VALUE
) {
    headerMain->set_title(
        VALUE
    );
}

void Header::set_subtitle(
    const Glib::ustring & VALUE
) {
    headerMain->set_subtitle(
        VALUE
    );
}