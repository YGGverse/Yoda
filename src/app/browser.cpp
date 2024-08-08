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
}

Browser::Header::Header()
{
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );
}

Browser::Container::Container()
{
    set_scrollable(
        SCROLLABLE
    );
}