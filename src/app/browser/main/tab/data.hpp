#ifndef APP_BROWSER_MAIN_TAB_DATA_HPP
#define APP_BROWSER_MAIN_TAB_DATA_HPP

#include <gtkmm/box.h>

namespace app::browser::main::tab
{
    class Data : public Gtk::Box
    {
        public:

            Data();

            ~Data();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_HPP