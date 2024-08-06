#ifndef APP_BROWSER_HEADER_MENU_MAIN_H
#define APP_BROWSER_HEADER_MENU_MAIN_H

// Dependencies
#include "../menu.h"

// Requirements
#include "main/debug.h"
#include "main/quit.h"

namespace app::browser::header
{
    class Menu;

    namespace menu
    {
        namespace main
        {
            class Debug;
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
                main::Debug *debug;
                main::Quit *quit;

                // Constructor
                Main(
                    Menu *menu
                );
        };
    };
};

#endif