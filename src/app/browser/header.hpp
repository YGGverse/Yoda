#ifndef APP_BROWSER_HEADER_H
#define APP_BROWSER_HEADER_H

#include <gtkmm/headerbar.h>

namespace app::browser
{
    namespace header
    {
        class Menu;
        class Tab;
    }

    class Header : public Gtk::HeaderBar
    {
        private:

            app::browser::header::Menu * menu;
            app::browser::header::Tab * tab;

        public:

            const bool SHOW_TITLE_BUTTONS = true;

            Header();

            ~Header();
    };
}

#endif // APP_BROWSER_HEADER_H