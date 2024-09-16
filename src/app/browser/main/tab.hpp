#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>
#include <sqlite3.h>

namespace app::browser::main
{
    namespace tab
    {
        class Label;
        class Page;
    }

    class Tab : public Gtk::Notebook
    {
        public:

            /*
             * Tab class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct Database
            {
                // APP_BROWSER_MAIN_TAB__*
                struct Session
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN__SESSION__ID,
                        TIME,
                        PAGE_NUMBER,
                        IS_CURRENT
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID,
                        const int & PAGE_NUMBER,
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

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__close_all,
                                            action__close,
                                            action__history_back,
                                            action__history_forward,
                                            action__reload,
                                            action__update;

            // Defaults
            static const bool REORDERABLE = true;
            static const bool SCROLLABLE = true;

        /*
         * Tab class API
         */
        public:

            Tab(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            // Actions
            int append(
                const bool & IS_CURRENT
            );

            void close(
                const int & PAGE_NUMBER
            );

            void close_left();
            void close_right();
            void close_all();

                void page_navigation_reload(
                    const int & PAGE_NUMBER,
                    const bool & ADD_HISTORY
                );

                void page_navigation_history_back(
                    const int & PAGE_NUMBER
                );

                void page_navigation_history_forward(
                    const int & PAGE_NUMBER
                );

            void update(
                const int & PAGE_NUMBER
            );

            int session_restore(
                const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
            ); // return sqlite3_finalize status code

            void session_save(
                const sqlite3_int64 & APP_BROWSER_MAIN__SESSION__ID
            );

            // Getters
            Glib::ustring get_page_title(
                const int & PAGE_NUMBER
            );

            Glib::ustring get_page_description(
                const int & PAGE_NUMBER
            );

            tab::Label * get_tabLabel(
                const int & PAGE_NUMBER
            );

            tab::Page * get_tabPage(
                const int & PAGE_NUMBER
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP