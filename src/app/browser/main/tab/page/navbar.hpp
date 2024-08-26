#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP

#include <giomm/simpleactiongroup.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>

namespace app::browser::main::tab::page
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
        // Components
        navbar::Base * navbarBase;
        navbar::Bookmark * navbarBookmark;
        navbar::History * navbarHistory;
        navbar::Request * navbarRequest;
        navbar::Update * navbarUpdate;

        // Defaults
        const int SPACING = 8;
        const int MARGIN = 8;

        public:

            Navbar(
                const Glib::ustring & request_text = ""
            );

            ~Navbar();

            // Actions
            void refresh();

            // Setters
            void set_request_text(
                const Glib::ustring & value
            );

            // Getters
            Glib::ustring get_request_text();

            Glib::ustring get_request_scheme();
            Glib::ustring get_request_host();
            Glib::ustring get_request_port();
            Glib::ustring get_request_path();
            Glib::ustring get_request_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP