#ifndef APP_BROWSER_HEADER_TAB_H
#define APP_BROWSER_HEADER_TAB_H

#include <glibmm/i18n.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Tab : public Gtk::MenuButton
    {
        public:

            const char* ICON = "tab-new-symbolic";
            const char* TOOLTIP = _("New tab");

            Tab();
    };
}

#endif // APP_BROWSER_HEADER_TAB_H