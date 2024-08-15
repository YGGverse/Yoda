#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_REQUEST_HPP

#include <glibmm/i18n.h>
#include <gtkmm/entry.h>

namespace app::browser::main::tab::data::navbar
{
    class Request : public Gtk::Entry
    {
        public:

            Request();

            ~Request();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_REQUEST_HPP