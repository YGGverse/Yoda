#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_MARKUP_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_MARKUP_HPP

#include <glibmm/markup.h>
#include <glibmm/ustring.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text::plain
{
    class Markup : public Gtk::Label
    {
        /*
         * Gemini class API
         */
        public:

            Markup(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_MARKUP_HPP