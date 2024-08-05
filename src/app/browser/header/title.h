#ifndef APP_BROWSER_HEADER_TITLE_H
#define APP_BROWSER_HEADER_TITLE_H

// Dependencies
#include "../header.h"

namespace app
{
    namespace browser
    {
        class Header;

        namespace header
        {
            class Title
            {
                public:

                    // GTK
                    GtkWidget *gtk;

                    // Defaults
                    const gint SPACING = 0;

                    // Dependencies
                    Header *header;

                    Title(
                        Header *header
                    );
            };
        }
    };
};

#endif