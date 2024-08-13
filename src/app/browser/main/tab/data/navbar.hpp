#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP

#include <gtkmm/box.h>

namespace app::browser::main::tab::data
{
    namespace navbar
    {
        class Base;
    }

    class Navbar : public Gtk::Box
    {
        private:

            navbar::Base * base;

        public:

            Navbar();

            ~Navbar();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP