#ifndef YODA_BROWSER_H
#define YODA_BROWSER_H

#include "../main.h"

namespace Yoda
{
    class Browser
    {
        public:

            GtkWidget *gtk;

            const guint WIDTH = 640;
            const guint HEIGHT = 480;
            const gchar* TITLE = "Yoda";

            Browser(
                GtkApplication *application
            );
    };
};

#endif