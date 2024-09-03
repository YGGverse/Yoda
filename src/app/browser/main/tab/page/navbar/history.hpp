#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP

#include <ctime>
#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <vector>

namespace app::browser::main::tab::page::navbar
{
    namespace history
    {
        class Back;
        class Forward;
    }

    class History : public Gtk::Box
    {
        // Extras
        struct Memory
        {
            Glib::ustring request;
            std::time_t time;      // event unix time
            bool permanent;        // save in database (on application close) @TODO
        };

        // Define navigation history storage
        std::vector<Memory> memory;
        int index = 0;

        // Private helpers
        Memory & get_memory_back();
        Memory & get_memory_forward();

        // Components
        history::Back * historyBack;
        history::Forward * historyForward;

        public:

            History();

            // Actions
            void back();
            void forward();

            void push(
                const Glib::ustring & REQUEST
            );

            void refresh();

            // Getters
            bool has_memory_back();
            bool has_memory_forward();

            // Copying getters (to keep private members encapsulation)
            Glib::ustring make_memory_back_request();
            Glib::ustring make_memory_forward_request();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP