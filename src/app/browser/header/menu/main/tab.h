#ifndef APP_BROWSER_HEADER_MENU_MAIN_TAB_H
#define APP_BROWSER_HEADER_MENU_MAIN_TAB_H

#include "../main.h"

namespace app::browser::header::menu
{
    class Main;

    namespace main
    {
        class Tab
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Gio
                GMenu* model;

                GMenuItem *item;

                // Dependencies
                Main *main;

                // Defaults
                const gchar *LABEL = "Tab";

                // Construct
                Tab(
                    Main *main
                );
        };
    };
};

#endif