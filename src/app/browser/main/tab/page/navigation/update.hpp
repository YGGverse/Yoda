#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_UPDATE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_UPDATE_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation
{
    class Update : public Gtk::Button
    {
        Glib::RefPtr<Gio::SimpleAction> action__update;

        public:

            Update(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            void refresh(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_UPDATE_HPP