#ifndef APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP
#define APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP

#include <gtkmm/box.h>

namespace app::browser::main::tab::data
{
    namespace navbar
    {
        class Base;
        class Bookmark;
        class History;
        class Update;
        class Request;
    }

    class Navbar : public Gtk::Box
    {
        private:

            // Defaults
            const int SPACING = 8;
            const int MARGIN = 8;

            // Components
            navbar::Base * base;
            navbar::Bookmark * bookmark;
            navbar::History * history;
            navbar::Request * request;
            navbar::Update * update;

        public:

            Navbar();

            ~Navbar();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_NAVBAR_HPP