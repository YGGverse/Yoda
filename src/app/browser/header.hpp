#ifndef APP_BROWSER_HEADER_HPP
#define APP_BROWSER_HEADER_HPP

#include <giomm/simpleaction.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/object.h>

namespace app::browser
{
    namespace header
    {
        class Main;
        class Menu;
        class Tab;
    }

    class Header : public Gtk::HeaderBar
    {
        // Components
        header::Main * headerMain;
        header::Menu * headerMenu;
        header::Tab * headerTab;

        // Defaults
        const bool SHOW_TITLE_BUTTONS = true;

        public:

            Header(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_APPEND,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
            );

            void refresh(
                const Glib::ustring & TITLE,
                const Glib::ustring & SUBTITLE
            );
    };
}

#endif // APP_BROWSER_HEADER_HPP