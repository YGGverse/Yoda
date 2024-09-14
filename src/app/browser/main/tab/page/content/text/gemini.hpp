#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP

#include <glibmm/markup.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/label.h>
#include <gtkmm/viewport.h>
#include <sstream>

namespace app::browser::main::tab::page::content::text
{
    class Gemini : public Gtk::Viewport
    {
        public:

            Gemini(
                const Glib::ustring & GEMTEXT
            );

            static Glib::ustring to_pango_markup(
                const Glib::ustring & GEMTEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP