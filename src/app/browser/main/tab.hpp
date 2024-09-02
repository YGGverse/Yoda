#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>
#include <gtkmm/object.h>

namespace app::browser::main
{
    namespace tab
    {
        class Label;
        class Page;
    }

    class Tab : public Gtk::Notebook
    {
        const bool REORDERABLE = true;
        const bool SCROLLABLE = true;

        tab::Label * get_tabLabel(
            const int & PAGE_NUMBER
        );

        tab::Page * get_tabPage(
            const int & PAGE_NUMBER
        );

        public:

            Tab();

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

            void back(
                const int & PAGE_NUMBER
            );

            void forward(
                const int & PAGE_NUMBER
            );

            void refresh(
                const int & PAGE_NUMBER
            );

            void update(
                const int & PAGE_NUMBER
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP