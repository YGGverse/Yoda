#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation::history
{
    class Forward : public Gtk::Button
    {
        Glib::RefPtr<Gio::SimpleAction> action__forward;

        public:

            Forward(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__FORWARD
            );

            void update(
                const bool & ENABLED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD_HPP