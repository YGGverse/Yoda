#ifndef APP_BROWSER_MAIN_TAB_LABEL_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/enums.h>
#include <sqlite3.h>

namespace app::browser::main::tab
{
    namespace label
    {
        class Pin;
        class Title;
    }

    class Label : public Gtk::Box
    {
        public:

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct Database
            {
                // app_browser_main_tab_label__*
                struct Session
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB__SESSION__ID,
                        TIME,
                        IS_PINNED
                    }; // table fields index

                    static int init(
                        sqlite3 * database
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
                        const bool & IS_PINNED
                    ); // return sqlite3_last_insert_rowid
                };
            };

        /*
         * Internal members
         */
        private:

            // Database
            sqlite3 * database;

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__tab_close;

            // Extras
            bool is_pinned;

            // Components
            label::Pin * labelPin;
            label::Title * labelTitle;

        /*
         * Class API
         */
        public:

            Label(
                sqlite3 * database,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE
            );

            // Actions
            int session_restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_finalize status code

            sqlite3_int64 session_save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_last_insert_rowid

            void pin();

            void update(
                const bool & IS_PINNED
            );

            void update(
                const Glib::ustring & TITLE
            );

            void update(
                const Glib::ustring & TITLE,
                const int & IS_PINNED
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_HPP