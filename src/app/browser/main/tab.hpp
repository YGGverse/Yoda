#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <gtkmm/widget.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    class Tab : public Gtk::Notebook
    {
        void on_switch(
            Gtk::Widget * page,
            guint page_num
        );

        public:

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

            void update(
                int number
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP