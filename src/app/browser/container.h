#ifndef APP_BROWSER_CONTAINER_H
#define APP_BROWSER_CONTAINER_H

#include "../browser.h"

namespace app
{
    class Browser;

    namespace browser
    {
        class Container
        {
            public:

                GtkWidget *gtk;

                const gint SPACING = 0;

                Container(
                    Browser *browser
                );
        };
    };
};

#endif