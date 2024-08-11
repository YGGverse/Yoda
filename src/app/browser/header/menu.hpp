#ifndef APP_BROWSER_HEADER_MENU_H
#define APP_BROWSER_HEADER_MENU_H

#include <giomm/menu.h>
#include <glibmm/i18n.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Menu : public Gtk::MenuButton
    {
        public:

            const char* TOOLTIP = _("Menu");

            Menu();
    };
}

#endif // APP_BROWSER_HEADER_MENU_H