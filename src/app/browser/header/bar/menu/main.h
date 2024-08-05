#ifndef APP_BROWSER_HEADER_BAR_MENU_MAIN_H
#define APP_BROWSER_HEADER_BAR_MENU_MAIN_H

// Dependencies
#include "../menu.h"

// Requirements
// ..

namespace app::browser::header::bar
{
    class Menu;

    namespace menu
    {
        class Main
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Dependencies
                Menu *menu;

                // Requirements
                // ..

                // Constructor
                Main(
                    Menu *menu
                );
        };
    };
};

#endif