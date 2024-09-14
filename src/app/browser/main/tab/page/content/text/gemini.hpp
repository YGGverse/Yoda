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
        /*
         * Tools (currently is private)
         */
        struct Line
        {
            struct Match
            {
                static bool link(
                    const Glib::ustring & GEMTEXT,
                    Glib::ustring & address,
                    Glib::ustring & date,
                    Glib::ustring & alt
                );
            };
        };

        struct Markup
        {
            struct Make
            {
                static Glib::ustring link(
                    const Glib::ustring & ADDRESS,
                    const Glib::ustring & DATE,
                    const Glib::ustring & ALT
                );
            };

            static Glib::ustring make(
                const Glib::ustring & GEMTEXT
            );
        };

        /*
         * Gemini class API
         */
        public:

            Gemini(
                const Glib::ustring & GEMTEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_HPP