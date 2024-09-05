#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/socketclient.h>
#include <giomm/socketconnection.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/regex.h>
#include <glibmm/stringutils.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>

namespace app::browser::main::tab
{
    namespace page
    {
        class Content;
        class Navigation;
        class Progress;
    }

    class Page : public Gtk::Box
    {
        // Extras
        Glib::ustring title;
        Glib::ustring subtitle;

        // Socket
        char buffer[0xfffff]; // 1Mb

        Glib::RefPtr<Gio::SocketClient> GioSocketClient;
        Glib::RefPtr<Gio::SocketConnection> GioSocketConnection;

        // Components
        page::Content * pageContent;
        page::Navigation * pageNavigation;
        page::Progress * pageProgress;

        public:

            Page(
                const Glib::ustring & TITLE,
                const Glib::ustring & SUBTITLE = "",
                const Glib::ustring & REQUEST = ""
            );

            // Getters
            Glib::ustring get_title();
            Glib::ustring get_subtitle();

            // Actions
            void refresh(
                const Glib::ustring & TITLE,
                const Glib::ustring & SUBTITLE,
                const double & PROGRESS
            );

            void navigation_update(
                const bool & HISTORY = false
            );

            bool navigation_history_try_back();
            bool navigation_history_try_forward();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP