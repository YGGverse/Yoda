#ifndef YODA_BROWSER_HEADER_H
#define YODA_BROWSER_HEADER_H

// Dependencies
#include "../Browser.h"

namespace YodaBrowser
{
    class Header
    {
        public:

            GtkWidget *gtk;

            const gboolean SHOW_TITLE_BUTTONS = true;

            Header();
    };
};

#endif