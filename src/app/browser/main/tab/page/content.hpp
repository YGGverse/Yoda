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

        void set_widget(
            Gtk::Widget * object
        );

        public:

            Content();

            ~Content();

            void set_text_gemini(
                const Glib::ustring & gemtext
            );

            void set_text_plain(
                const Glib::ustring & text
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP