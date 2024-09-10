#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP

#include <giomm/simpleaction.h>
#include <glibmm/refptr.h>
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
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__NAVIGATION_UPDATE
            );

            // Actions
            void refresh(
                const double & PROGRESS_FRACTION
            );

            int save();

            void history_add(
                const Glib::ustring & REQUEST,
                const bool & UPDATE_MEMORY_INDEX
            );

            // Actionable getters
            bool try_history_back(
                Glib::ustring & request,
                const bool & UPDATE_MEMORY_INDEX
            );

            bool try_history_forward(
                Glib::ustring & request,
                const bool & UPDATE_MEMORY_INDEX
            );

            // Getters
            Glib::ustring get_request_text();

            Glib::ustring get_request_scheme();
            Glib::ustring get_request_host();
            Glib::ustring get_request_port();
            Glib::ustring get_request_path();
            Glib::ustring get_request_query();

            // Setters
            void set_request_text(
                const Glib::ustring & VALUE
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_HPP