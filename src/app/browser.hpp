#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
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
        // Components
        app::browser::Header * browserHeader;
        app::browser::Main * browserMain;

        // Defaults
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