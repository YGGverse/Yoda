#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <glibmm/i18n.h>
#include <gtkmm/applicationwindow.h>

namespace lib
{
    class Database;
}

namespace app
{
    namespace browser
    {
        class Header;
        class Main;
    }

    class Browser : public Gtk::ApplicationWindow
    {
        private:

            app::browser::Header * header;
            app::browser::Main * main;

        public:

            Browser(
                //const Glib::RefPtr<Gtk::Application> & app,
                //const std::shared_ptr<lib::Database> & db
            );

            ~Browser();
    };
}

#endif // APP_BROWSER_HPP