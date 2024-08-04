#ifndef YODA_BROWSER_HEADER_H
#define YODA_BROWSER_HEADER_H

#include "../../main.h"

#include "../Browser.h"

namespace YodaBrowser
{
    class Header
    {
        public:

            // GTK
            GtkWidget *gtk;

            // Dependencies
            Yoda::Browser *browser;

            // Defaults
            const gboolean SHOW_TITLE_BUTTONS = true;

            // Construct
            Header(
                Yoda::Browser *browser
            );
    };
};

#endif