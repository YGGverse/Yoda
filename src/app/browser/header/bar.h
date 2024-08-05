#ifndef APP_BROWSER_HEADER_BAR_H
#define APP_BROWSER_HEADER_BAR_H

// Dependencies
#include "../header.h"

namespace app
{
    namespace browser
    {
        class Header;

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

                    Bar(
                        Header *header
                    );
            };
        }
    };
};

#endif