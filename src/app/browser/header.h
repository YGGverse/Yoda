#ifndef APP_BROWSER_HEADER_H
#define APP_BROWSER_HEADER_H

#include "../browser.h"

namespace app
{
    class Browser;

    namespace browser
    {
        class Header
        {
            public:

                GtkWidget *gtk;

                const gboolean SHOW_TITLE_BUTTONS = true;

                Header(
                    Browser *browser
                );
        };
    };
};

#endif