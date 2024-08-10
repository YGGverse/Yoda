#include "browser.hpp"
#include "browser/header.hpp"

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
        * new browser::Header()
    );
}