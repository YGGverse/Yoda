#ifndef YODA_BROWSER_H
#define YODA_BROWSER_H

// Dependencies
#include "../main.h"

// Requirements
#include "Browser/Header.h"

namespace Yoda
{
    class Browser
    {
        public:

            GtkWidget *gtk;

            const guint WIDTH = 640;
            const guint HEIGHT = 480;

            Browser(
                GtkApplication *application
            );
    };
};

#endif