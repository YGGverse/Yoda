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
        // Components
        history::Back * historyBack;
        history::Forward * historyForward;

        int index = 0;

        public:

            // Extras
            struct Memory
            {
                Glib::ustring request;
                std::time_t time;      // event unix time
                bool permanent;        // save in database (on application close) @TODO
            };

            // Define navigation history storage
            std::vector<Memory> memory;

            History();

            // Actions
            bool try_back(
                Memory & match,
                bool follow = true
            );

            bool try_forward(
                Memory & match,
                bool follow = true
            );

            void push(
                const Glib::ustring & REQUEST
            );

            void refresh();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP