#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP

#include <ctime>
#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <vector>

namespace app::browser::main::tab::page::navigation
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

        int index = -1;

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
            void add(
                const Glib::ustring & REQUEST,
                const bool & UPDATE_MEMORY_INDEX
            );

            void refresh();

            void save(); // @TODO save history to the permanent storage

            bool try_back(
                Memory & match,
                const bool & UPDATE_MEMORY_INDEX
            );

            bool try_forward(
                Memory & match,
                const bool & UPDATE_MEMORY_INDEX
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HISTORY_HPP