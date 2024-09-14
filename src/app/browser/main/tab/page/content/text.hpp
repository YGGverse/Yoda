#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/scrolledwindow.h>

namespace app::browser::main::tab::page::content
{
    class Text : public Gtk::ScrolledWindow
    {
        /*
         * Private members
         */
        Glib::ustring title;

        public:

            /*
            * Extra features
            */
            enum Type
            {
                GEMINI,
                PLAIN
            };

            /*
            * Text class API
            */
            Text(
                const Type & TYPE,
                const Glib::ustring & SOURCE,
                GUri * uri
            );

            // Getters
            Glib::ustring get_title();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP