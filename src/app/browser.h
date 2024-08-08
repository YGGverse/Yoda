#ifndef APP_BROWSER_H
#define APP_BROWSER_H

#include <gtkmm/window.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/notebook.h>

namespace app
{
    class Browser : public Gtk::Window
    {
        public:

            const char* TITLE = "Basic application";
            const guint WIDTH = 640;
            const guint HEIGHT = 480;

            Browser();

        class Header : Gtk::HeaderBar
        {

        };

        class Container : Gtk::Notebook
        {

        };
    };
}

#endif // APP_BROWSER_H