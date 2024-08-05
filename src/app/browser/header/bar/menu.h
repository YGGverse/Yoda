#ifndef APP_BROWSER_HEADER_BAR_MENU_H
#define APP_BROWSER_HEADER_BAR_MENU_H

// Dependencies
#include "../bar.h"

// Requirements
#include "menu/main.h"

namespace app::browser::header
{
    class Bar;

    namespace bar
    {
        namespace menu
        {
            class Main;
        };

        class Menu
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Dependencies
                Bar *bar;

                // Requirements
                menu::Main *main;

                // Constructor
                Menu(
                    Bar *bar
                );
        };
    };
};

#endif