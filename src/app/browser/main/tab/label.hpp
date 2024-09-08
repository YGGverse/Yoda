#ifndef APP_BROWSER_MAIN_TAB_LABEL_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/gestureclick.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab
{
    class Label : public Gtk::Label
    {
        Glib::RefPtr<Gio::SimpleAction> action__close;

        public:

            Label(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_HPP