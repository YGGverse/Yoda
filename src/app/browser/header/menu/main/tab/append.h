#ifndef APP_BROWSER_HEADER_MENU_MAIN_TAB_APPEND_H
#define APP_BROWSER_HEADER_MENU_MAIN_TAB_APPEND_H

#include "../tab.h"

namespace app::browser::header::menu::main
{
    class Tab;

    namespace tab
    {
        class Append
        {
            public:

                // GTK
                GMenuItem *item;

                // Gio
                GSimpleAction *action;

                // Dependencies
                Tab *tab;

                // Defaults
                const gchar *LABEL = "New tab";

                const gchar *ACCEL_1 = "<Control>t";
                const gchar *ACCEL_2 = "<Control>T";

                const gchar *ACTION_NS = "app.%s";
                const gchar *ACTION_ID = "browser.header.menu.main.tab.append.activate";

                // Construct
                Append(
                    Tab *tab
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