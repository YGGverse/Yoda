#ifndef APP_BROWSER_HEADER_BAR_H
#define APP_BROWSER_HEADER_BAR_H

// Dependencies
#include "../header.h"

// Requirements
#include "bar/menu.h"

namespace app::browser
{
    class Header;

    namespace header
    {
        namespace bar
        {
            class Menu;
        };

        class Bar
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Defaults
                const gint SPACING = 0;

                // Dependencies
                Header *header;

                // Requirements
                bar::Menu *menu;

                Bar(
                    Header *header
                );
        };
    };
};

#endif