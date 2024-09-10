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
#include <sqlite3.h>

namespace app::browser::main::tab
{
    namespace page
    {
        class Content;
        class Navigation;
    }

    class Page : public Gtk::Box
    {
        public:

            enum class MIME
            {
                TEXT_PLAIN,
                TEXT_GEMINI,
                UNDEFINED
            };

            struct DB
            {
                struct APP_BROWSER_MAIN_TAB_PAGE__SESSION
                {
                    enum
                    {
                        ID,
                        TIME,
                        MIME,
                        TITLE,
                        DESCRIPTION
                    };

                    static int init(
                        sqlite3 * db
                    );

                    static int clean(
                        sqlite3 * db,
                        const int & DB__APP_BROWSER_MAIN_TAB__SESSION_ID
                    );

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB__SESSION_ID,
                        const Page::MIME & MIME,
                        const Glib::ustring & TITLE,
                        const Glib::ustring & DESCRIPTION
                    );
                };
            };

        private:

            // Meta
            MIME mime;
            Glib::ustring title;
            Glib::ustring description;
            double progress_fraction;

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__refresh;

            // Database
            sqlite3 * db;

            // Socket
            char buffer[0xfffff]; // 1Mb

            Glib::RefPtr<Gio::SocketClient> GioSocketClient;
            Glib::RefPtr<Gio::SocketConnection> GioSocketConnection;

            // Components
            page::Content * pageContent;
            page::Navigation * pageNavigation;

        public:

            Page(
                sqlite3 * db,
                const MIME & MIME,
                const Glib::ustring & TITLE,
                const Glib::ustring & DESCRIPTION,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_UPDATE
            );

            // Actions
            void refresh();

            int save(
                const sqlite3_int64 & DB__APP_BROWSER_MAIN_TAB__SESSION_ID
            );

            void update(
                const MIME & MIME,
                const Glib::ustring & TITLE,
                const Glib::ustring & DESCRIPTION,
                const double & PROGRESS_FRACTION
            );

            void navigation_update(
                const bool & ADD_HISTORY
            );

            void navigation_history_back();
            void navigation_history_forward();

            // Getters
            MIME get_mime();
            Glib::ustring get_title();
            Glib::ustring get_description();

            Glib::ustring get_navigation_request_text();

            // Setters
            void set_navbar_request_text(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP