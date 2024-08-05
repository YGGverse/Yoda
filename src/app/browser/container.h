#ifndef APP_BROWSER_CONTAINER_H
#define APP_BROWSER_CONTAINER_H

// Dependencies
#include "../browser.h"

// Requirements
#include "container/tab.h"

namespace app
{
    class Browser;

    namespace browser
    {
        namespace container
        {
            class Tab;
        }

        class Container
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Defaults
                const gint SPACING = 0;

                // Requirements
                container::Tab *tab;

                Container(
                    Browser *browser
                );
        };
    };
};

#endif