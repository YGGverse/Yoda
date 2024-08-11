#ifndef APP_BROWSER_H
#define APP_BROWSER_H

#include <glibmm/i18n.h>
#include <gtkmm/applicationwindow.h>

namespace app
{
    class Browser : public Gtk::ApplicationWindow
    {
        public:

            const char * TITLE = _("Yoda");
            const int WIDTH = 640;
            const int HEIGHT = 480;

            Browser(
                const Glib::RefPtr<Gtk::Application> & app
            );

            void debug();
    };
}

#endif // APP_BROWSER_H