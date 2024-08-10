#include "header.hpp"
#include "header/menu.hpp"
#include "header/tab.hpp"

using namespace app::browser;

Header::Header()
{
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );

    pack_start(
        * new header::Menu()
    );

    pack_start(
        * new header::Tab()
    );
}