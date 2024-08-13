#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_BACK_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_BACK_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::data::navbar::history
{
    class Back : public Gtk::Button
    {
        public:

            Back();

            ~Back();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_BACK_HPP