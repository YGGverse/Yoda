#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    class Tab : public Gtk::Notebook
    {
        const bool REORDERABLE = true;
        const bool SCROLLABLE = true;

        public:

            Tab();

            ~Tab();

            Glib::ustring get_label_text(
                int page_number
            );

            void append(
                const Glib::ustring & page_navbar_request_text = "",
                bool focus = true
            );

            void close(
                int page_number
            );

            void close_left();
            void close_right();
            void close_all();

            void update(
                int page_number
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP