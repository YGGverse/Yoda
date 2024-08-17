#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP

#include <glibmm/i18n.h>
#include <glibmm/regex.h>
#include <glibmm/ustring.h>
#include <gtkmm/entry.h>

namespace app::browser::main::tab::page::navbar
{
    class Request : public Gtk::Entry
    {
        Glib::ustring scheme,
                      host,
                      port,
                      path,
                      query;

        void parse();

        public:

            Request();

            ~Request();

            Glib::ustring get_scheme();
            Glib::ustring get_host();
            Glib::ustring get_port();
            Glib::ustring get_path();
            Glib::ustring get_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP