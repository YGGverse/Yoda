#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation
{
    class Base : public Gtk::Button
    {
        public:

            Base();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP