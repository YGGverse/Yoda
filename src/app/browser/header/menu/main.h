#ifndef APP_BROWSER_HEADER_MENU_MAIN_H
#define APP_BROWSER_HEADER_MENU_MAIN_H

// Dependencies
#include "../menu.h"

// Requirements
#include "main/tab.h"
#include "main/debug.h"
#include "main/quit.h"

namespace app::browser::header
{
    class Menu;

    namespace menu
    {
        namespace main
        {
            class Tab;
            class Debug;
            class Quit;
        };

        class Main
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Gio
                GMenu* model;

                // Dependencies
                Menu *menu;

                // Requirements
                main::Tab *tab;
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