#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP

#include <glibmm/i18n.h>
#include <gtkmm/entry.h>

#include <regex>
#include <string>

namespace app::browser::main::tab::page::navbar
{
    class Request : public Gtk::Entry
    {
        private:

            std::string scheme,
                        host,
                        port,
                        path,
                        query;

            void parse();

        public:

            Request();

            ~Request();

            std::string get_scheme();
            std::string get_host();
            std::string get_port();
            std::string get_path();
            std::string get_query();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_REQUEST_HPP