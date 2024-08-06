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

        gtk_button_set_icon_name(
            GTK_BUTTON(
                this->gtk
            ),
            Tab::ICON
        );

        gtk_widget_set_tooltip_text(
            GTK_WIDGET(
                this->gtk
            ),
            Tab::TOOLTIP
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}
