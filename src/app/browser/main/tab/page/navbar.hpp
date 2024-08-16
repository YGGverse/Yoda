#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP

#include <giomm/simpleactiongroup.h>
#include <glibmm/refptr.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <sigc++/functors/mem_fun.h>
#include <string>

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
        private:

            // Actions
            Glib::RefPtr<Gio::SimpleActionGroup> action_group;

            // Components
            navbar::Base * base;
            navbar::Bookmark * bookmark;
            navbar::History * history;
            navbar::Request * request;
            navbar::Update * update;

            // Defaults
            const int SPACING = 8;
            const int MARGIN = 8;

        public:

            Navbar();
            ~Navbar();

            // Actions
            void refresh();

            // Getters
            Glib::ustring get_request();

            std::string get_request_scheme();
            std::string get_request_host();
            std::string get_request_port();
            std::string get_request_path();
            std::string get_request_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_HPP