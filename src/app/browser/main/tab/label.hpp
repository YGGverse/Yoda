#ifndef APP_BROWSER_MAIN_TAB_LABEL_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_HPP

#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/gestureclick.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab
{
    class Label : public Gtk::Label
    {
        public:

            Label(
                const Glib::ustring & TEXT
            );

            ~Label();
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_HPP