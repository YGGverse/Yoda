#ifndef APP_BROWSER_HEADER_TAB_H
#define APP_BROWSER_HEADER_TAB_H

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::header
{
    class Tab : public Gtk::Button
    {
        public:

            const char* ICON = "tab-new-symbolic";
            const char* TOOLTIP = _("New tab");

            Tab();

            ~Tab();
    };
}

#endif // APP_BROWSER_HEADER_TAB_H