#include "title.h"

namespace app
{
    namespace browser
    {
        namespace header
        {
            // Construct
            Title::Title(
                Header *header
            ) {
                // Init GTK
                this->gtk = gtk_box_new(
                    GTK_ORIENTATION_HORIZONTAL,
                    Title::SPACING
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
