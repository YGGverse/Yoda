#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    class Tab : public Gtk::Notebook
    {
        public:

            Tab();

            ~Tab();

            Glib::ustring get_label_text(
                int page_number
            );

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