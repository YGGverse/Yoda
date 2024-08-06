#ifndef APP_BROWSER_HEADER_MENU_H
#define APP_BROWSER_HEADER_MENU_H

// Dependencies
#include "../header.h"

// Requirements
#include "menu/main.h"

namespace app::browser
{
    class Header;

    namespace header
    {
        namespace menu
        {
            class Main;
        }

        class Menu
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Dependencies
                Header *header;

                // Requirements
                menu::Main *main;

                // Defaults
                const gchar *TOOLTIP = "Menu";

                // Constructor
                Menu(
                    Header *header
                );
        };
    };
};

#endif