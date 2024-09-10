#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation::history
{
    class Back : public Gtk::Button
    {
        Glib::RefPtr<Gio::SimpleAction> action__back;

        public:

            Back(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__BACK
            );

            void update(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK_HPP