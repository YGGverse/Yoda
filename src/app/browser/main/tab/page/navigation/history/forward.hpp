#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation::history
{
    class Forward : public Gtk::Button
    {
        public:

            Forward();

            void refresh(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP