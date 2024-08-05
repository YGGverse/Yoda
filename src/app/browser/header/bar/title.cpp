#include "title.h"

namespace app::browser::header::bar
{
    // Construct
    Title::Title(
        Bar *bar
    ) {
        // Init dependencies
        this->bar = bar;

        // Init GTK
        this->gtk = gtk_label_new(
            Title::LABEL
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}