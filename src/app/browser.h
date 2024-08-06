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

            // Dependencies
            GtkApplication *app;

            // Requirements
            browser::Header *header;

            // Defaults
            const guint WIDTH = 640;
            const guint HEIGHT = 480;

            // Constructor
            Browser(
                GtkApplication *application
            );

        private:

            // Events
            static void _shutdown(
                GtkApplication *application
            );
    };
};

#endif