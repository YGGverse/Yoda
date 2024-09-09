#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <giomm/simpleaction.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/application.h>
#include <gtkmm/applicationwindow.h>
#include <gtkmm/object.h>
#include <sqlite3.h>

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
                sqlite3 * db,
                const Glib::RefPtr<Gtk::Application> & APP
            );
    };
}

#endif // APP_BROWSER_HPP