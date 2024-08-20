#ifndef APP_BROWSER_HEADER_MAIN_TITLE_HPP
#define APP_BROWSER_HEADER_MAIN_TITLE_HPP

#include <glibmm/ustring.h>
#include <gtkmm/label.h>

namespace app::browser::header::main
{
    class Title : public Gtk::Label
    {
        public:

            Title();

            ~Title();

            void set_title(
                Glib::ustring $value
            );

            void set_subtitle(
                Glib::ustring $value
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_TITLE_HPP