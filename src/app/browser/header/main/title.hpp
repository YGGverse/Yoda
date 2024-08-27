#ifndef APP_BROWSER_HEADER_MAIN_TITLE_HPP
#define APP_BROWSER_HEADER_MAIN_TITLE_HPP

#include <glibmm/ustring.h>
#include <gtkmm/enums.h>
#include <gtkmm/label.h>
#include <pangomm/layout.h>

namespace app::browser::header::main
{
    class Title : public Gtk::Label
    {
        const int WIDTH_CHARS = 5;

        const Glib::ustring DEFAULT_TEXT = "Yoda";

        public:

            Title();

            ~Title();

            void set(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_TITLE_HPP