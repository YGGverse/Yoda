#ifndef APP_BROWSER_HEADER_HPP
#define APP_BROWSER_HEADER_HPP

#include <glibmm/ustring.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/object.h>

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
        // Components
        header::Main * headerMain;
        header::Menu * headerMenu;
        header::Tab * headerTab;

        // Defaults
        const bool SHOW_TITLE_BUTTONS = true;

        public:

            Header();

            void set_title(
                const Glib::ustring & VALUE
            );

            void set_subtitle(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_HEADER_HPP