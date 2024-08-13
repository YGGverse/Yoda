#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_UPDATE_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_UPDATE_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::data::navbar
{
    class Update : public Gtk::Button
    {
        public:

            Update();

            ~Update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_UPDATE_HPP