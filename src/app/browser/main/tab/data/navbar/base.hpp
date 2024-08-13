#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_BASE_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_BASE_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::data::navbar
{
    class Base : public Gtk::Button
    {
        public:

            Base();

            ~Base();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_BASE_HPP