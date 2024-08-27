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
                const int & PAGE_NUMBER
            );

            void append(
                const Glib::ustring & TITLE,
                const Glib::ustring & REQUEST = "",
                const bool & FOCUS = true
            );

            void close(
                const int & PAGE_NUMBER
            );

            void close_left();
            void close_right();
            void close_all();

            void refresh(
                const int & PAGE_NUMBER
            );

            void update(
                const int & PAGE_NUMBER
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP