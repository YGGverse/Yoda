#ifndef APP_BROWSER_HEADER_H
#define APP_BROWSER_HEADER_H

// Dependencies
#include "../browser.h"

// Requirements
#include "header/bar.h"
#include "header/tab.h"

namespace app
{
    class Browser;

    namespace browser
    {
        namespace header
        {
            class Bar;
            class Tab;
        }

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
                header::Bar *bar;
                header::Tab *tab;

                Header(
                    Browser *browser
                );
        };
    };
};

#endif