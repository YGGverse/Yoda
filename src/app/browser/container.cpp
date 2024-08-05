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
        }
    }
}
