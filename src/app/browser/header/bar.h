#ifndef APP_BROWSER_HEADER_BAR_H
#define APP_BROWSER_HEADER_BAR_H

// Dependencies
#include "../header.h"

// Requirements
#include "../menu.h"
#include "bar/title.h"

namespace app
{
    namespace browser
    {
        class Header;

        class Menu;

        namespace header
        {
            namespace bar
            {
                class Title;
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
                    Menu *menu;
                    bar::Title *title;

                    Bar(
                        Header *header
                    );
            };
        };
    };
};

#endif