#include "container.h"

namespace app
{
    namespace browser
    {
        // Construct
        Container::Container(
            Browser *browser
        ) {
            // Init GTK
            this->gtk = gtk_box_new(
                GTK_ORIENTATION_VERTICAL,
                Container::SPACING
            );

            // Init tabs component
            this->tab = new container::Tab(
                this
            );

            gtk_box_append(
                GTK_BOX(
                    this->gtk
                ),
                GTK_WIDGET(
                    this->tab->gtk
                )
            );

            // @TODO append testing tab
            this->tab->append(
                NULL,
                false,
                true
            );

            // Render
            gtk_widget_show(
                GTK_WIDGET(
                    this->gtk
                )
            );
        }
    }
}
