#ifndef APP_BROWSER_HEADER_BAR_TITLE_H
#define APP_BROWSER_HEADER_BAR_TITLE_H

// Dependencies
#include "../bar.h"

namespace app
{
    namespace browser
    {
        namespace header
        {
            class Bar;

            namespace bar
            {
                class Title
                {
                    public:

                        // GTK
                        GtkWidget *gtk;

                        // Defaults
                        const char* LABEL = "Yoda";

                        // Dependencies
                        Bar *bar;

                        Title(
                            Bar *bar
                        );
                };
            };
        };
    };
};

#endif