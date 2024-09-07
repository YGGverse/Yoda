#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation::history
{
    class Back : public Gtk::Button
    {
        public:

            Back();

            void refresh(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP