#ifndef APP_BROWSER_MAIN_HPP
#define APP_BROWSER_MAIN_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <sqlite3.h>

namespace app::browser
{
    namespace main
    {
        class Tab;
    }

    class Main : public Gtk::Box
    {
        public:

            /*
             * Tab class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        APP_BROWSER__SESSION__ID,
                        TIME
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER__SESSION__ID
                    ); // return sqlite3_last_insert_rowid

                };
            };

        /*
         * Internal members
         */
        private:

            // Database
            sqlite3 * db;

            // Components
            main::Tab * mainTab;

            // Defaults
            const bool HOMOGENEOUS = true;

        /*
         * Main class API
         */
        public:

            Main(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            // Actions
            void tab_append();

            void tab_close_all();
            void tab_close_left();
            void tab_close_right();
            void tab_close();

                void tab_page_navigation_reload();
                void tab_page_navigation_history_back();
                void tab_page_navigation_history_forward();

            int restore(
                const sqlite3_int64 & APP_BROWSER__SESSION__ID
            ); // return sqlite3_finalize status code

            void save(
                const sqlite3_int64 & APP_BROWSER__SESSION__ID
            );

            void update();

            // Getters
            Glib::ustring get_tab_page_title();
            Glib::ustring get_tab_page_description();
    };
}

#endif // APP_BROWSER_MAIN_HPP