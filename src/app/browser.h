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

            const Glib::ustring TITLE = "Basic application";
            const int WIDTH = 640;
            const int HEIGHT = 480;

            Browser();

        class Header : Gtk::HeaderBar
        {
            public:

                const bool SHOW_TITLE_BUTTONS = true;

                Header();
        };

        class Container : Gtk::Notebook
        {
            public:

                const bool SCROLLABLE = true;
                const bool REORDERABLE = true;

                Container();
        };
    };
}

#endif // APP_BROWSER_H