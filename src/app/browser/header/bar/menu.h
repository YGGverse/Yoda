#ifndef APP_BROWSER_HEADER_BAR_MENU_H
#define APP_BROWSER_HEADER_BAR_MENU_H

#include "../bar.h"

namespace app::browser::header
{
    class Bar;

    namespace bar
    {
        class Menu
        {
            public:

                GtkWidget *gtk;

                Menu(
                    Bar *bar
                );
        };
    };
};

#endif