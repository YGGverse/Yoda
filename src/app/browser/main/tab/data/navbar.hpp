#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP

#include <gtkmm/box.h>

namespace app::browser::main::tab::data
{
    class Navbar : public Gtk::Box
    {
        public:

            Navbar();

            ~Navbar();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP