#include "header.hpp"
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
}

Header::~Header()
{
    delete menu;
    delete tab;
}