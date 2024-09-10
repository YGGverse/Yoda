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
             * Class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct DB
            {
                // APP_BROWSER_MAIN_TAB__*
                struct SESSION
                {
                    enum
                    {
                        ID,
                        TIME,
                        PAGE_NUMBER,
                        IS_CURRENT,
                        LABEL_TEXT
                    }; // table fields index

                    static int init(
                        sqlite3 * db
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * db
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * db,
                        const int & PAGE_NUMBER,
                        const bool & IS_CURRENT,
                        const Glib::ustring & LABEL_TEXT
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
            Glib::RefPtr<Gio::SimpleAction> action__refresh,
                                            action__tab_close_active,
                                            action__tab_close_all,
                                            action__tab_page_navigation_history_back,
                                            action__tab_page_navigation_history_forward,
                                            action__tab_page_navigation_update;

            // Defaults
            const bool REORDERABLE = true;
            const bool SCROLLABLE = true;

        /*
         * Class API
         */
        public:

            Tab(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ACTIVE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_UPDATE
            );

            // Actions
            int append(
                const Glib::ustring & LABEL_TEXT,
                const bool & IS_CURRENT
            );

            void close(
                const int & PAGE_NUMBER
            );

            void close_left();
            void close_right();
            void close_all();

                void page_navigation_update(
                    const int & PAGE_NUMBER,
                    const bool & ADD_HISTORY
                );

                void page_navigation_history_back(
                    const int & PAGE_NUMBER
                );

                void page_navigation_history_forward(
                    const int & PAGE_NUMBER
                );

            void refresh(
                const int & PAGE_NUMBER
            );

            int restore();

            void clean();

            void save();

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