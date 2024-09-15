#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/simpleaction.h>
#include <giomm/simpleactiongroup.h>
#include <giomm/socketclient.h>
#include <giomm/socketconnection.h>
#include <glibmm/i18n.h>
#include <glibmm/main.h>
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

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN_TAB_PAGE__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB__SESSION__ID,
                        TIME,
                        MIME,
                        TITLE,
                        DESCRIPTION
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
                        const Page::MIME & MIME,
                        const Glib::ustring & TITLE,
                        const Glib::ustring & DESCRIPTION
                    ); // return sqlite3_last_insert_rowid
                };
            };

        /*
         * Internal members
         */
        private:

            // Meta
            MIME mime;

            double progress_fraction;

            GUri * uri;

            Glib::ustring title;
            Glib::ustring description;

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__update;

            // Database
            sqlite3 * db;

            // Socket
            char buffer[0xfffff]; // 1Mb

            Glib::RefPtr<Gio::SocketClient> GioSocketClient;
            Glib::RefPtr<Gio::SocketConnection> GioSocketConnection;

            // Components
            page::Content * pageContent;
            page::Navigation * pageNavigation;

        /*
         * Class API
         */
        public:

            Page(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            // Actions
            int restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_finalize status code

            void save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            );

            void update();

            void navigation_reload(
                const bool & ADD_HISTORY
            );

            void navigation_history_back();
            void navigation_history_forward();

            // Getters
            MIME get_mime();
            Glib::ustring get_title();
            Glib::ustring get_description();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP