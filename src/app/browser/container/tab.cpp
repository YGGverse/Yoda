#include "tab.h"

namespace app
{
    namespace browser
    {
        namespace container
        {
            // Construct
            Tab::Tab(
                Container *container
            ) {
                // Init GTK
                this->gtk = gtk_notebook_new();

                gtk_notebook_set_scrollable(
                    GTK_NOTEBOOK(
                        this->gtk
                    ),
                    Tab::SCROLLABLE
                );

                gtk_widget_show(
                    GTK_WIDGET(
                        this->gtk
                    )
                );
            }
        }
    }
}
