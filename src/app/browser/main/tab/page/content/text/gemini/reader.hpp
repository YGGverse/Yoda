#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_READER_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_READER_HPP

#include <glibmm/i18n.h>
#include <glibmm/markup.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/cssprovider.h>
#include <gtkmm/enums.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text::gemini
{
    class Reader : public Gtk::Label
    {
        /*
         * Tools (currently is private)
         */
        struct Line
        {
            struct Match
            {
                static bool header(
                    const Glib::ustring & GEMTEXT,
                    int & level,
                    Glib::ustring & text
                );

                static bool link(
                    const Glib::ustring & GEMTEXT,
                    Glib::ustring & address,
                    Glib::ustring & date,
                    Glib::ustring & alt
                );

                static bool quote(
                    const Glib::ustring & GEMTEXT,
                    Glib::ustring & text
                );
            };
        };

        struct Make
        {
            static Glib::ustring header(
                const int & LEVEL,
                const Glib::ustring & TEXT
            );

            static Glib::ustring link(
                const Glib::ustring & ADDRESS,
                const Glib::ustring & DATE,
                const Glib::ustring & ALT
            );

            static Glib::ustring plain(
                const Glib::ustring & TEXT
            );

            static Glib::ustring quote(
                const Glib::ustring & TEXT
            );
        };

        Glib::ustring make(
            const Glib::ustring & GEMTEXT
        );

        /*
         * Private members
         */
        Glib::ustring title;

        /*
         * Reader class API
         */
        public:

            Reader(
                const Glib::ustring & GEMTEXT
            );

            // Getters
            Glib::ustring get_title();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_GEMINI_READER_HPP