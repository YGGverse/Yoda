#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_MARKUP_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_MARKUP_HPP

#include <glibmm/markup.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text::gemini
{
    class Markup : public Gtk::Label
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

        /*
         * Gemini class API
         */
        public:

            Markup(
                const Glib::ustring & GEMTEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_MARKUP_HPP