#ifndef APP_BROWSER_HEADER_MENU_HPP
#define APP_BROWSER_HEADER_MENU_HPP

#include <giomm/menu.h>
#include <glibmm/i18n.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Menu : public Gtk::MenuButton
    {
        Glib::RefPtr<Gio::Menu> tab,
                                tab_close,
                                tool,
                                main;

        public:

            Menu();

            ~Menu();
    };
}

#endif // APP_BROWSER_HEADER_MENU_HPP