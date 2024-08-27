#ifndef APP_BROWSER_HEADER_MAIN_HPP
#define APP_BROWSER_HEADER_MAIN_HPP

#include <glibmm/ustring.h>
#include <gtkmm/box.h>

namespace app::browser::header
{
    namespace main
    {
        class Title;
        class Subtitle;
    }

    class Main : public Gtk::Box
    {
        main::Title * mainTitle;
        main::Subtitle * mainSubtitle;

        const bool HOMOGENEOUS = true;

        public:

            Main();

            ~Main();

            void set_title(
                const Glib::ustring & TEXT
            );

            void set_subtitle(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_HPP