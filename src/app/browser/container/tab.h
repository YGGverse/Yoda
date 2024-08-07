#pragma once

#ifndef APP_BROWSER_CONTAINER_TAB_H
#define APP_BROWSER_CONTAINER_TAB_H

// Dependencies
#include "../container.h"

// Requirements
#include "page.h"

namespace app
{
    namespace browser
    {
        class Container;

        namespace container
        {
            class Page; // @TODO not required here

            class Tab
            {
                public:

                    // GTK
                    GtkWidget *gtk;

                    // Defaults
                    const gboolean REORDERABLE = true;
                    const gboolean SCROLLABLE = true;

                    // Dependencies
                    Container *container;

                    Tab(
                        Container *container
                    );

                    void append(
                        char *request,
                        bool open,
                        bool focus
                    );
            };
        };
    };
};

#endif