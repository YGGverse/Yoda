#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP

#include <glibmm/ustring.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text
{
    class Gemini : public Gtk::Label
    {
        public:

            Gemini(
                const Glib::ustring & GEMTEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP