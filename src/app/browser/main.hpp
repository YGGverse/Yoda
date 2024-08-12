#ifndef APP_BROWSER_MAIN_HPP
#define APP_BROWSER_MAIN_HPP

#include <gtkmm/box.h>

namespace app::browser
{
    namespace main
    {
        class Tab;
    }

    class Main : public Gtk::Box
    {
        private:

            app::browser::main::Tab * tab;

        public:

            Main();

            ~Main();

            void tab_append();

            void tab_close();
            void tab_close_left();
            void tab_close_right();
            void tab_close_all();
    };
}

#endif // APP_BROWSER_MAIN_HPP