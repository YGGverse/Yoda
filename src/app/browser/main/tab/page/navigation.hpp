#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP

#include <giomm/simpleaction.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <sqlite3.h>

namespace app::browser::main::tab::page
{
    namespace navigation
    {
        class Base;
        class Bookmark;
        class History;
        class Reload;
        class Request;
    }

    class Navigation : public Gtk::Box
    {
        public:

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID,
                        TIME
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
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
            navigation::Base * navigationBase;
            navigation::Bookmark * navigationBookmark;
            navigation::History * navigationHistory;
            navigation::Reload * navigationReload;
            navigation::Request * navigationRequest;

            // Defaults
            const int SPACING = 8;
            const int MARGIN = 8;

        /*
         * Class API
         */
        public:

            Navigation(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_RELOAD
            );

            // Actions
            void update(
                const double & PROGRESS_FRACTION
            );

            int restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
            ); // return sqlite3_finalize status code

            void save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_PAGE__SESSION__ID
            );

            void history_add(
                const Glib::ustring & REQUEST,
                const bool & UPDATE_MEMORY_INDEX
            );

            // Actionable getters
            bool try_history_back(
                Glib::ustring & request,
                const bool & UPDATE_MEMORY_INDEX
            );

            bool try_history_current(
                Glib::ustring & request
            );

            bool try_history_forward(
                Glib::ustring & request,
                const bool & UPDATE_MEMORY_INDEX
            );

            // Getters
            Glib::ustring get_request_text();

            Glib::ustring get_request_scheme();
            Glib::ustring get_request_host();
            Glib::ustring get_request_port();
            Glib::ustring get_request_path();
            Glib::ustring get_request_query();

            // Setters
            void set_request_text(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP