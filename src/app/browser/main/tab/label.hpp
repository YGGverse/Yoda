#ifndef APP_BROWSER_MAIN_TAB_LABEL_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/gestureclick.h>
#include <gtkmm/label.h>
#include <sqlite3.h>

namespace app::browser::main::tab
{
    class Label : public Gtk::Label
    {
        public:

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN_TAB_LABEL__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB__SESSION__ID,
                        TIME,
                        TEXT
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
            Glib::RefPtr<Gio::SimpleAction> action__tab_close;

        /*
         * Class API
         */
        public:

            Label(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE
            );

            // Actions
            int restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_finalize status code

            int save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_finalize status code
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_HPP