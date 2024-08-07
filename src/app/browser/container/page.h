#pragma once

#ifndef APP_BROWSER_CONTAINER_PAGE_H
#define APP_BROWSER_CONTAINER_PAGE_H

#include "../container.h"

namespace app
{
    namespace browser
    {
        class Container;

        namespace container
        {
            class Page
            {
                public:

                    GtkWidget *gtk;

                    const gint SPACING = 0;

                    Page(
                        Container *container
                    );

                    void init(
                        char *request,
                        bool focus
                    );

                    void open(
                        char *request,
                        bool history
                    );
            };
        };
    };
};

#endif