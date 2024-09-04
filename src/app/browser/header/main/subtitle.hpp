#ifndef APP_BROWSER_HEADER_MAIN_SUBTITLE_HPP
#define APP_BROWSER_HEADER_MAIN_SUBTITLE_HPP

#include <glibmm/ustring.h>
#include <gtkmm/enums.h>
#include <gtkmm/label.h>
#include <pangomm/layout.h>

namespace app::browser::header::main
{
    class Subtitle : public Gtk::Label
    {
        const int WIDTH_CHARS = 5;

        public:

            Subtitle();

            void set(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_SUBTITLE_HPP