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
            };
        };
    };
};

#endif