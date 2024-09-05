#ifndef APP_BROWSER_MAIN_HPP
#define APP_BROWSER_MAIN_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>

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

            Main();

            // Getters
            Glib::ustring get_current_tab_page_title();
            Glib::ustring get_current_tab_page_subtitle();

            // Actions
            void tab_append();
            void tab_close_all();
            void tab_close_left();
            void tab_close_right();
            void tab_close();

            void tab_page_update();

            bool tab_page_navigation_history_try_back();
            bool tab_page_navigation_history_try_forward();

            void refresh();
    };
}

#endif // APP_BROWSER_MAIN_HPP