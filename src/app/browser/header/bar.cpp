#include "bar.h"

namespace app
{
    namespace browser
    {
        namespace header
        {
            // Construct
            Bar::Bar(
                Header *header
            ) {
                // Init GTK
                this->gtk = gtk_box_new(
                    GTK_ORIENTATION_HORIZONTAL,
                    Bar::SPACING
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
