#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP

#include <glibmm/ustring.h>
#include <gtkmm/scrolledwindow.h>

namespace app::browser::main::tab::page::content
{
    class Text : public Gtk::ScrolledWindow
    {
        public:

            Text();

            void set_gemini(
                const Glib::ustring & GEMTEXT
            );

            void set_plain(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP