#ifndef APP_BROWSER_MAIN_H
#define APP_BROWSER_MAIN_H

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
    };
}

#endif // APP_BROWSER_MAIN_H