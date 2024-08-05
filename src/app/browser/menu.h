#ifndef APP_BROWSER_MENU_H
#define APP_BROWSER_MENU_H

#include "../browser.h"

namespace app
{
    class Browser;

    namespace browser
    {
        class Menu
        {
            public:

                GtkWidget *gtk;

                Menu(
                    Browser *browser
                );
        };
    };
};

#endif