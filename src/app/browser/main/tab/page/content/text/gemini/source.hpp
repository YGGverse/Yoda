#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_SOURCE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_SOURCE_HPP

#include <glibmm/markup.h>
#include <glibmm/ustring.h>
#include <gtkmm/enums.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text::gemini
{
    class Source : public Gtk::Label
    {
        /*
         * Gemini class API
         */
        public:

            Source(
                const Glib::ustring & GEMTEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_SOURCE_HPP