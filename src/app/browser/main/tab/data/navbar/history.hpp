#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_HPP

#include <glibmm/i18n.h>
#include <gtkmm/box.h>

namespace app::browser::main::tab::data::navbar
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

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HISTORY_HPP