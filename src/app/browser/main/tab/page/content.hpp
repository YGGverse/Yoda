#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP

#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/widget.h>

namespace app::browser::main::tab::page
{
    class Content : public Gtk::Box
    {
        Gtk::Widget * widget;

        void update(
            Gtk::Widget * new_widget
        );

        public:

            Content();

            ~Content();

            void text_gemini(
                const Glib::ustring & gemtext
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP