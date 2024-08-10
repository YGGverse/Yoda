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

                    class Page
                    {
                        public:

                            class Navbar
                            {
                                public:

                                    Navbar();
                            };

                            class Body
                            {
                                public:

                                    Body();
                            };

                            Page();

                        private:

                            Navbar _navbar;
                            Body _body;
                    };

                    const bool SCROLLABLE = true;
                    const bool REORDERABLE = true;

                    Container();

                    Page append(
                        char* request,
                        bool open = true,
                        bool focus = false
                    );

                    void update();
            };
    };
}

#endif // APP_BROWSER_H