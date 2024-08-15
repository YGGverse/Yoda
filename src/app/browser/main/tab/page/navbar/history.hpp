#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP

#include <glibmm/i18n.h>
#include <gtkmm/box.h>

namespace app::browser::main::tab::page::navbar
{
    namespace history
    {
        class Back;
        class Forward;
    }

    class History : public Gtk::Box
    {
        private:

            history::Back * back;
            history::Forward * forward;

        public:

            History();

            ~History();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HISTORY_HPP