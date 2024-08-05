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

            gtk_widget_show(
                GTK_WIDGET(
                    this->gtk
                )
            );

            // Init tab
            this->tab = new container::Tab(
                this
            );

            gtk_box_append(
                GTK_BOX(
                    this->gtk
                ),
                GTK_WIDGET(
                    this->tab
                )
            );
        }
    }
}
