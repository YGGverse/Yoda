#ifndef APP_BROWSER_HEADER_HPP
#define APP_BROWSER_HEADER_HPP

#include <glibmm/ustring.h>
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

            void set_title(
                const Glib::ustring text
            );

            void set_subtitle(
                const Glib::ustring text
            );
    };
}

#endif // APP_BROWSER_HEADER_HPP