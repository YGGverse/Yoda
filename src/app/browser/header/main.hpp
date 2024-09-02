#ifndef APP_BROWSER_HEADER_MAIN_HPP
#define APP_BROWSER_HEADER_MAIN_HPP

#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>

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

            void set_title(
                const Glib::ustring & VALUE
            );

            void set_subtitle(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_HEADER_MAIN_HPP