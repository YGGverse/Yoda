#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    namespace tab
    {
        class Page;
    }

    class Tab : public Gtk::Notebook
    {
        const bool REORDERABLE = true;
        const bool SCROLLABLE = true;

        tab::Page * get_tab_page(
            const int & PAGE_NUMBER
        );

        public:

            Tab();

            ~Tab();

            Glib::ustring get_page_title(
                const int & PAGE_NUMBER
            );

            Glib::ustring get_page_subtitle(
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