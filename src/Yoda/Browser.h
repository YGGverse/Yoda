#ifndef YODA_BROWSER_H
#define YODA_BROWSER_H

#include "../main.h"

class YodaBrowser
{
    public:

        GtkWidget *gtk;

        const guint WIDTH = 640;
        const guint HEIGHT = 480;
        const gchar* TITLE = "Yoda";

        YodaBrowser(
            GtkApplication *application
        );
};

#endif