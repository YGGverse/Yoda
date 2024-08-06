#ifndef APP_BROWSER_HEADER_BAR_MENU_MAIN_H
#define APP_BROWSER_HEADER_BAR_MENU_MAIN_H

// Dependencies
#include "../menu.h"

// Requirements
#include "main/quit.h"

namespace app::browser::header::bar
{
    class Menu;

    namespace menu
    {
        namespace main
        {
            class Quit;
        };

        class Main
        {
            private:

                GMenu* _model;

            public:

                // GTK
                GtkWidget *gtk;

                // Dependencies
                Menu *menu;

                // Requirements
                main::Quit *quit;

                // Constructor
                Main(
                    Menu *menu
                );
        };
    };
};

#endif