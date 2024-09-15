#ifndef APP_BROWSER_HEADER_TAB_HPP
#define APP_BROWSER_HEADER_TAB_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/button.h>

namespace app::browser::header
{
    class Tab : public Gtk::Button
    {
        Glib::RefPtr<Gio::SimpleAction> action__tab_append;

        public:

            Tab(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_APPEND
            );
    };
}

#endif // APP_BROWSER_HEADER_TAB_HPP