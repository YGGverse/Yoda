#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/simpleaction.h>
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
    }

    class Page : public Gtk::Box
    {
        // Extras
        enum class MIME
        {
            TEXT_PLAIN,
            TEXT_GEMINI,
            UNDEFINED
        };

        MIME mime;
        Glib::ustring title;
        Glib::ustring subtitle;

        // Socket
        char buffer[0xfffff]; // 1Mb

        Glib::RefPtr<Gio::SocketClient> GioSocketClient;
        Glib::RefPtr<Gio::SocketConnection> GioSocketConnection;

        // Components
        page::Content * pageContent;
        page::Navigation * pageNavigation;

        public:

            Page(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_UPDATE
            );

            // Actions
            void refresh(
                const Glib::ustring & TITLE,
                const Glib::ustring & SUBTITLE,
                const double & PROGRESS_FRACTION
            );

            void navigation_update(
                const bool & ADD_HISTORY
            );

            void navigation_history_back();
            void navigation_history_forward();

            // Getters
            Page::MIME get_mime();
            Glib::ustring get_title();
            Glib::ustring get_subtitle();

            Glib::ustring get_navigation_request_text();

            // Setters
            void set_navbar_request_text(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP