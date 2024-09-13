#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP

//#include <gtkmm/adjustment.h> @TODO
#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/scrolledwindow.h>
#include <gtkmm/viewport.h>

namespace app::browser::main::tab::page::content
{
    namespace text
    {
        class Gemini;
        class Plain;
    }

    class Text : public Gtk::ScrolledWindow
    {
        text::Gemini * textGemini;
        text::Plain * textPlain;

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

            ~Text();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_HPP