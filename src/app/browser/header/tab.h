#pragma once

#ifndef APP_BROWSER_HEADER_TAB_H
#define APP_BROWSER_HEADER_TAB_H

// Dependencies
#include "../header.h"

namespace app::browser
{
    class Header;

    namespace header
    {
        class Tab
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Dependencies
                Header *header;

                // Defaults
                const gchar *ICON = "tab-new-symbolic";
                const gchar *TOOLTIP = "New tab";

                // Constructor
                Tab(
                    Header *header
                );
        };
    };
};

#endif