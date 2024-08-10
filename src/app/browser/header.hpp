#ifndef APP_BROWSER_HEADER_H
#define APP_BROWSER_HEADER_H

#include <gtkmm/headerbar.h>

namespace app::browser
{
    class Header : public Gtk::HeaderBar
    {
        public:

            const bool SHOW_TITLE_BUTTONS = true;

            Header();
    };
}

#endif // APP_BROWSER_HEADER_H