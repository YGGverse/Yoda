#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_FORWARD_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_FORWARD_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navbar::history
{
    class Forward : public Gtk::Button
    {
        public:

            Forward();

            ~Forward();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_FORWARD_HPP