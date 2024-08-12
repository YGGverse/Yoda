#ifndef APP_BROWSER_H
#define APP_BROWSER_H

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

            const char * TITLE = _("Yoda");
            const int WIDTH = 640;
            const int HEIGHT = 480;

            Browser(
                const Glib::RefPtr<Gtk::Application> & app,
                const lib::Database & db
            );

            ~Browser();

            void mainTabAppend();

            void debug();
    };
}

#endif // APP_BROWSER_H