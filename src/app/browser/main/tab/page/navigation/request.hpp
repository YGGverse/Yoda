#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP

#include <glibmm/i18n.h>
#include <glibmm/main.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/entry.h>

namespace app::browser::main::tab::page::navigation
{
    class Request : public Gtk::Entry
    {
        // Extras
        double progress_fraction;

        Glib::ustring scheme,
                      host,
                      port,
                      path,
                      query;

        // Defaults
        const bool HEXPAND = true;
        const double PROGRESS_PULSE_STEP = .1;
        const int PROGRESS_ANIMATION_TIME = 10;

        // Private helpers
        void parse();

        public:

            Request(
                const Glib::ustring & VALUE = "" // @TODO remove default value
            );

            // Actions
            void refresh(
                const double & PROGRESS_FRACTION
            );

            // Getters
            Glib::ustring get_scheme();
            Glib::ustring get_host();
            Glib::ustring get_port();
            Glib::ustring get_path();
            Glib::ustring get_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_REQUEST_HPP