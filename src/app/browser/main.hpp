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
        // Components
        main::Tab * mainTab;

        // Defaults
        const bool HOMOGENEOUS = true;

        public:

            Main(
                sqlite3 * db,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
            );

            // Actions
            void refresh();
            void restore();
            void save();

                void tab_append();

                void tab_close_all();
                void tab_close_left();
                void tab_close_right();
                void tab_close();

                    void tab_page_navigation_update();

                    void tab_page_navigation_history_back();
                    void tab_page_navigation_history_forward();

            // Getters
            Glib::ustring get_current_tab_page_title();
            Glib::ustring get_current_tab_page_subtitle();
    };
}

#endif // APP_BROWSER_MAIN_HPP