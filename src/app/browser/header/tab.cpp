#include "tab.h"

namespace app::browser::header
{
    // Construct
    Tab::Tab(
        Header *header
    ) {
        // Init dependencies
        this->header = header;

        // Init GTK
        this->gtk = gtk_button_new();

        gtk_button_set_label (
            GTK_BUTTON(
                this->gtk
            ),
            Tab::LABEL
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}
