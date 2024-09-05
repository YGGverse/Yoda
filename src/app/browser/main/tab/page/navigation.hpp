#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP

#include <giomm/simpleactiongroup.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>

namespace app::browser::main::tab::page
{
    namespace navigation
    {
        class Base;
        class Bookmark;
        class History;
        class Update;
        class Request;
    }

    class Navigation : public Gtk::Box
    {
        // Components
        navigation::Base * navigationBase;
        navigation::Bookmark * navigationBookmark;
        navigation::History * navigationHistory;
        navigation::Request * navigationRequest;
        navigation::Update * navigationUpdate;

        // Defaults
        const int SPACING = 8;
        const int MARGIN = 8;

        public:

            Navigation(
                const Glib::ustring & REQUEST
            );

            // Actions
            void history_add(
                const Glib::ustring & REQUEST
            );

            void refresh();

            // Setters
            void set_request_text(
                const Glib::ustring & VALUE
            );

            // Getters
            Glib::ustring get_request_text();

            Glib::ustring get_request_scheme();
            Glib::ustring get_request_host();
            Glib::ustring get_request_port();
            Glib::ustring get_request_path();
            Glib::ustring get_request_query();

            bool try_history_back(
                Glib::ustring & request
            );

            bool try_history_forward(
                Glib::ustring & request
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP