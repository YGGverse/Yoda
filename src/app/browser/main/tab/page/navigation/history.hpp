#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP

#include <ctime>
#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <sqlite3.h>
#include <vector>

namespace app::browser::main::tab::page::navigation
{
    namespace history
    {
        class Back;
        class Forward;
    }

    class History : public Gtk::Box
    {
        private:
            /*
             * History class database
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
                        REQUEST,
                        IS_CURRENT
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
                        const int & TIME,
                        const Glib::ustring & REQUEST,
                        const bool & IS_CURRENT
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
            history::Back * historyBack;
            history::Forward * historyForward;

            // Extras
            int index = -1;

        /*
         * History class API
         */
        public:

            // Extras
            struct Memory
            {
                Glib::ustring request;
                std::time_t time; // event unix time
            };

            // Define navigation history storage
            std::vector<Memory> memory;

            History(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD
            );

            // Actions
            void add(
                const Glib::ustring & REQUEST,
                const bool & UPDATE_MEMORY_INDEX
            );

            int restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
            ); // return sqlite3_finalize status code

            void save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__SESSION__ID
            );

            void update();

            bool try_back(
                Memory & match,
                const bool & UPDATE_MEMORY_INDEX
            );

            bool try_forward(
                Memory & match,
                const bool & UPDATE_MEMORY_INDEX
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP