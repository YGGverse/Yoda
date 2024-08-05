#ifndef APP_BROWSER_HEADER_BAR_H
#define APP_BROWSER_HEADER_BAR_H

// Dependencies
#include "../header.h"

// Requirements
#include "../menu.h"

namespace app
{
    namespace browser
    {
        class Header;

        class Menu;

        namespace header
        {
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
                    Menu *menu;

                    Bar(
                        Header *header
                    );
            };
        };
    };
};

#endif