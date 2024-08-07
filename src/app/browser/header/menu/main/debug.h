#pragma once

#ifndef APP_BROWSER_HEADER_MENU_MAIN_DEBUG_H
#define APP_BROWSER_HEADER_MENU_MAIN_DEBUG_H

#include "../main.h"

namespace app::browser::header::menu
{
    class Main;

    namespace main
    {
        class Debug
        {
            public:

                // GTK
                GMenuItem *item;

                GSimpleAction *action;

                // Dependencies
                Main *main;

                // Defaults
                const gchar *LABEL = "Debug";

                const gchar *ACCEL_1 = "<Control><Shift>i";
                const gchar *ACCEL_2 = "<Control><Shift>I";

                const gchar *ACTION_NS = "app.%s";
                const gchar *ACTION_ID = "browser.header.menu.main.debug.activate";

                // Construct
                Debug(
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