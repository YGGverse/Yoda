#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
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
            struct DB
            {
                // APP_BROWSER__*
                struct SESSION
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
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
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
            sqlite3 * db;

            // Components
            app::browser::Header * browserHeader;
            app::browser::Main * browserMain;

            // Defaults
            const int WIDTH = 640;
            const int HEIGHT = 480;
            const bool IS_FULLSCREEN = false;

        /*
         * Browser class API
         */
        public:

            Browser(
                sqlite3 * db
            );

            // Actions
            int restore(); // return sqlite3_finalize status code

            void clean();

            void save();
    };
}

#endif // APP_BROWSER_HPP