#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/main.h>
#include <glibmm/refptr.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/entry.h>
#include <sqlite3.h>

namespace app::browser::main::tab::page::navigation
{
    class Request : public Gtk::Entry
    {
        public:

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        TIME,
                        TEXT
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const int & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID,
                        const Glib::ustring & TEXT
                    ); // return sqlite3_last_insert_rowid
                };
            };

        /*
         * Internal members
         */
        private:

            // Database
            sqlite3 * db;

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__refresh,
                                            action__update;

            // Extras
            double progress_fraction;

            Glib::ustring scheme,
                        host,
                        port,
                        path,
                        query;

            // Defaults
            const bool HEXPAND = true;
            const double PROGRESS_PULSE_STEP = .1;
            const int PROGRESS_ANIMATION_TIME = 10;

            // Private helpers
            void parse();

        /*
         * Class API
         */
        public:

            Request(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            // Actions
            void refresh(
                const double & PROGRESS_FRACTION
            );

            int save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION_ID
            );

            // Getters
            Glib::ustring get_scheme();
            Glib::ustring get_host();
            Glib::ustring get_port();
            Glib::ustring get_path();
            Glib::ustring get_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP