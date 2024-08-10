#ifndef APP_BROWSER_H
#define APP_BROWSER_H

#include <glibmm/i18n.h>

#include <gtkmm/application.h>
#include <gtkmm/applicationwindow.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/menubutton.h>
#include <gtkmm/notebook.h>

namespace app
{
    class Browser : public Gtk::ApplicationWindow
    {
        public:

            const char* TITLE = _("Yoda");
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

                            const char* TOOLTIP = _("Menu");

                            Menu();
                    };

                    class Tab : public Gtk::MenuButton
                    {
                        public:

                            const char* ICON = "tab-new-symbolic";
                            const char* TOOLTIP = _("New tab");

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