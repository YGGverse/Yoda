#ifndef APP_BROWSER_MAIN_HPP
#define APP_BROWSER_MAIN_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>

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
            ~Main();

            // Getters
            Glib::ustring get_current_tab_page_title();
            Glib::ustring get_current_tab_page_subtitle();

            // Actions
            void tab_append();
            void tab_close_all();
            void tab_close_left();
            void tab_close_right();
            void tab_close();
            void tab_history_back();
            void tab_history_forward();
            void tab_update();

            void refresh();
    };
}

#endif // APP_BROWSER_MAIN_HPP