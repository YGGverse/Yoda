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
                // Init dependencies
                this->header = header;

                // Init GTK
                this->gtk = gtk_box_new(
                    GTK_ORIENTATION_HORIZONTAL,
                    Bar::SPACING
                );

                // Init menu
                this->menu = new Menu(
                    this->header->browser
                );

                gtk_box_append(
                    GTK_BOX(
                        this->gtk
                    ),
                    GTK_WIDGET(
                        this->menu->gtk
                    )
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
}
