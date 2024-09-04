#ifndef APP_BROWSER_HEADER_TAB_HPP
#define APP_BROWSER_HEADER_TAB_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::header
{
    class Tab : public Gtk::Button
    {
        public:

            Tab();
    };
}

#endif // APP_BROWSER_HEADER_TAB_HPP