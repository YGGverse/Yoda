#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <glibmm/i18n.h>
#include <gtkmm/applicationwindow.h>
#include <gtkmm/object.h>

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
        app::browser::Header * browserHeader;
        app::browser::Main * browserMain;

        const int WIDTH = 640;
        const int HEIGHT = 480;

        public:

            Browser(
                //const Glib::RefPtr<Gtk::Application> & app,
                //const std::shared_ptr<lib::Database> & db
            );
    };
}

#endif // APP_BROWSER_HPP