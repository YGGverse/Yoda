#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    namespace tab
    {
        class Label;
    }

    class Tab : public Gtk::Notebook
    {
        public:

            tab::Label * label;

            Tab();

            ~Tab();

            void append(
                const char * request,
                bool open,
                bool focus
            );

            void close(
                int number
            );

            void close_left();
            void close_right();
            void close_all();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP