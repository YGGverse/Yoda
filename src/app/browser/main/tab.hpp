#ifndef APP_BROWSER_MAIN_TAB_HPP
#define APP_BROWSER_MAIN_TAB_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/notebook.h>

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

            // Getters
            Glib::ustring get_page_title(
                const int & PAGE_NUMBER
            );

            Glib::ustring get_page_subtitle(
                const int & PAGE_NUMBER
            );

            // Actions
            void refresh(
                const int & PAGE_NUMBER // @TODO
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

                void page_navigation_update(
                    const int & PAGE_NUMBER
                );

                bool page_navigation_history_try_back(
                    const int & PAGE_NUMBER
                );

                bool page_navigation_history_try_forward(
                    const int & PAGE_NUMBER
                );
    };
}

#endif // APP_BROWSER_MAIN_TAB_HPP