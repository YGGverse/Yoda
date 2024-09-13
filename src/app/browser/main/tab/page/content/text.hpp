#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP

//#include <gtkmm/adjustment.h> @TODO
#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/scrolledwindow.h>

namespace app::browser::main::tab::page::content
{
    class Text : public Gtk::ScrolledWindow
    {
        public:

            enum Type
            {
                GEMINI,
                PLAIN
            };

            Text(
                const Type & TYPE,
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP