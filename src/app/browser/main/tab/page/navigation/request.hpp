#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/main.h>
#include <glibmm/refptr.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/entry.h>
#include <sigc++/connection.h>
#include <sqlite3.h>

namespace app::browser::main::tab::page::navigation
{
    class Request : public Gtk::Entry
    {
        public:

            /*
             * Request class database
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
                        APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
                        TIME,
                        TEXT
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID,
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
            Glib::RefPtr<Gio::SimpleAction> action__update,
                                            action__reload;

            // Extras
            double progress_fraction;
            sigc::connection progress_connection;

            // Defaults
            const bool HEXPAND = true;
            const double PROGRESS_PULSE_STEP = .1;
            const int PROGRESS_ANIMATION_TIME = 10;

        /*
         * Class API
         */
        public:

            Request(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD
            );

            // Actions
            void update(
                const double & PROGRESS_FRACTION
            );

            void update(
                const Glib::ustring & TEXT,
                const double & PROGRESS_FRACTION
            );

            int restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
            ); // return sqlite3_finalize status code

            int save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP