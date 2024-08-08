#ifndef APP_BROWSER_H
#define APP_BROWSER_H

#include <gtkmm/window.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/menubutton.h>
#include <gtkmm/notebook.h>

namespace app
{
    class Browser : public Gtk::Window
    {
        public:

            const Glib::ustring TITLE = "Yoda";
            const int WIDTH = 640;
            const int HEIGHT = 480;

            Browser();

            class Header : public Gtk::HeaderBar
            {
                public:

                    const bool SHOW_TITLE_BUTTONS = true;

                    Header();

                    class Menu : public Gtk::MenuButton
                    {
                        public:

                            const Glib::ustring TOOLTIP = "Menu";

                            Menu();
                    };

                    class Tab : public Gtk::MenuButton
                    {
                        public:

                            const Glib::ustring TOOLTIP = "New tab";

                            Tab();
                    };
            };

            class Container : public Gtk::Notebook
            {
                public:

                    const bool SCROLLABLE = true;
                    const bool REORDERABLE = true;

                    Container();
            };
    };
}

#endif // APP_BROWSER_H