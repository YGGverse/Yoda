#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/varianttype.h>
#include <gtkmm/application.h>
#include <gtkmm/applicationwindow.h>
#include <gtkmm/object.h>
#include <sqlite3.h>

namespace app
{
    namespace browser
    {
        class Header;
        class Main;
    }

    class Browser : public Gtk::ApplicationWindow
    {
        public:

            /*
             * Tab class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct Database
            {
                // app_browser__*
                struct Session
                {
                    enum
                    {
                        ID,
                        TIME,
                        WIDTH,
                        HEIGHT,
                        IS_FULLSCREEN
                    }; // table fields index

                    static int init(
                        sqlite3 * database
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * database
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * database,
                        const int & WIDTH,
                        const int & HEIGHT,
                        const bool & IS_FULLSCREEN
                    ); // return sqlite3_last_insert_rowid
                };
            };

        /*
         * Internal members
         */
        private:

            // Database
            sqlite3 * database;

            // Components
            app::browser::Header * browserHeader;
            app::browser::Main * browserMain;

            // Defaults
            static const int WIDTH = 640;
            static const int HEIGHT = 480;
            static const bool IS_FULLSCREEN = false;

        /*
         * Browser class API
         */
        public:

            Browser(
                sqlite3 * database
            );

            // Actions
            int session_restore(); // return sqlite3_finalize status code
            void session_clean();
            void session_save();
    };
}

#endif // APP_BROWSER_HPP