#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP

#include <glibmm/ustring.h>
#include <gtkmm/viewport.h>

namespace app::browser::main::tab::page::content::text
{
    class Gemini : public Gtk::Viewport
    {
        /*
         * Gemini class API
         */
        public:

            Gemini(
                const Glib::ustring & REQUEST,
                const Glib::ustring & GEMTEXT,
                Glib::ustring & title
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP