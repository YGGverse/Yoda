#ifndef APP_BROWSER_HEADER_MAIN_HPP
#define APP_BROWSER_HEADER_MAIN_HPP

#include <glibmm/ustring.h>
#include <gtkmm/box.h>

namespace app::browser::header
{
    namespace main
    {
        class Title;
    }

    class Main : public Gtk::Box
    {
        main::Title * title;

        public:

            Main();

            ~Main();

            void set_title(
                const Glib::ustring text
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_HPP