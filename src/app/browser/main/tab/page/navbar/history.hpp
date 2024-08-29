#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP

#include "glibmm/ustring.h"
#include <glibmm/i18n.h>
#include <gtkmm/box.h>
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
        // Memory
        std::vector<Glib::ustring> history;
        std::vector<Glib::ustring>::iterator index;

        // Components
        history::Back * back;
        history::Forward * forward;

        public:

            History();

            ~History();

            void refresh();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP