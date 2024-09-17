#ifndef APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/label.h>
#include <pangomm/layout.h>
#include <sqlite3.h>

namespace app::browser::main::tab::label
{
    class Title : public Gtk::Label
    {
        public:

            /*
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct Database
            {
                // app_browser_main_tab_label_title__*
                struct Session
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID,
                        TIME,
                        TEXT
                    }; // table fields index

                    static int init(
                        sqlite3 * database
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID,
                        Glib::ustring & TEXT
                    ); // return sqlite3_last_insert_rowid
                };
            };

        /*
         * Internal members
         */
        private:

            // Database
            sqlite3 * database;

            // Extras
            Glib::ustring text;

            // Defaults
            static const int WIDTH_CHARS = 16;

        /*
         * Class API
         */
        public:

            Title(
                sqlite3 * database
            );

            // Actions
            int session_restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
            ); // return sqlite3_finalize status code

            sqlite3_int64 session_save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB_LABEL__SESSION__ID
            ); // return sqlite3_last_insert_rowid

            void update(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP