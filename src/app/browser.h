#ifndef APP_BROWSER_H
#define APP_BROWSER_H

// Dependencies
#include "../main.h"

// Requirements
#include "browser/header.h"

namespace app
{
    namespace browser
    {
        class Header;
    }

    class Browser
    {
        public:

            // GTK
            GtkWidget *gtk;

            // Defaults
            const guint WIDTH = 640;
            const guint HEIGHT = 480;

            // Requirements
            browser::Header *header;

            Browser(
                GtkApplication *application
            );
    };
};

#endif