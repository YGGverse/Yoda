#ifndef APP_BROWSER_HEADER_HPP
#define APP_BROWSER_HEADER_HPP

#include <gtkmm/headerbar.h>

namespace app::browser
{
    namespace header
    {
        class Main;
        class Menu;
        class Tab;
    }

    class Header : public Gtk::HeaderBar
    {
        app::browser::header::Main * main;
        app::browser::header::Menu * menu;
        app::browser::header::Tab * tab;

        public:

            Header();

            ~Header();
    };
}

#endif // APP_BROWSER_HEADER_HPP