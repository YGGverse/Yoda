#ifndef APP_BROWSER_HEADER_H
#define APP_BROWSER_HEADER_H

// Dependencies
#include "../browser.h"

// Requirements
#include "menu.h"

namespace app
{
    class Browser;

    namespace browser
    {
        class Menu;

        class Header
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Defaults
                const gboolean SHOW_TITLE_BUTTONS = true;

                // Dependencies
                Browser *browser;

                // Requirements
                Menu *menu;

                Header(
                    Browser *browser
                );
        };
    };
};

#endif