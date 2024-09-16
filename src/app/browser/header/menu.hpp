#ifndef APP_BROWSER_HEADER_MENU_HPP
#define APP_BROWSER_HEADER_MENU_HPP

#include <giomm/menu.h>
#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Menu : public Gtk::MenuButton
    {
        static Glib::ustring get_action_detailed_name(
            const Glib::RefPtr<Gio::SimpleAction> & ACTION
        );

        public:

            Menu(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_CLEAN,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_RESTORE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_SAVE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_APPEND,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PIN,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_RELOAD
            );
    };
}

#endif // APP_BROWSER_HEADER_MENU_HPP