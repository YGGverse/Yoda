#include "browser.h"

using namespace app;

Browser::Browser()
{
    set_title(
        TITLE
    );

    set_default_size(
        WIDTH,
        HEIGHT
    );

    set_titlebar(
        * new Header()
    );
}

Browser::Header::Header()
{
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );

    pack_start(
        * new Menu()
    );

    pack_start(
        * new Tab()
    );
}

Browser::Header::Menu::Menu()
{
    set_tooltip_text(
        TOOLTIP
    );
}

Browser::Header::Tab::Tab()
{
    set_tooltip_text(
        TOOLTIP
    );
}

Browser::Container::Container()
{
    set_scrollable(
        SCROLLABLE
    );
}