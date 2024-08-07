#pragma once

#ifndef APP_BROWSER_HEADER_MENU_MAIN_QUIT_H
#define APP_BROWSER_HEADER_MENU_MAIN_QUIT_H

#include "../main.h"

namespace app::browser::header::menu
{
    class Main;

    namespace main
    {
        class Quit
        {
            public:

                // GTK
                GMenuItem *item;

                GSimpleAction *action;

                // Dependencies
                Main *main;

                // Defaults
                const gchar *LABEL = "Quit";

                const gchar *ACCEL_1 = "<Control>q";
                const gchar *ACCEL_2 = "<Control>Q";

                const gchar *ACTION_NS = "app.%s";
                const gchar *ACTION_ID = "browser.header.menu.main.quit.activate";

                // Construct
                Quit(
                    Main *main
                );

            private:

                // Events
                static void _activate(
                    GSimpleAction* action,
                    GVariant* parameter,
                    gpointer user_data
                );
        };
    };
};

#endif