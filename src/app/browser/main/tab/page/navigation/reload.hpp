#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_RELOAD_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_RELOAD_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation
{
    class Reload : public Gtk::Button
    {
        Glib::RefPtr<Gio::SimpleAction> action__reload;

        public:

            Reload(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD
            );

            void update(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_RELOAD_HPP