#ifndef APP_BROWSER_CONTAINER_TAB_H
#define APP_BROWSER_CONTAINER_TAB_H

#include "../container.h"

namespace app
{
    namespace browser
    {
        class Container;

        namespace container
        {
            class Tab
            {
                public:

                    GtkWidget *gtk;

                    const gboolean REORDERABLE = true;
                    const gboolean SCROLLABLE = true;

                    Tab(
                        Container *container
                    );
            };
        };
    };
};

#endif