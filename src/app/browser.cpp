#include "browser.h"

namespace app
{
    // Construct
    Browser::Browser(
        GtkApplication *application
    ) {
        // Init dependencies
        this->application = application;

        // Init GTK
        this->gtk = gtk_application_window_new(
            GTK_APPLICATION(
                this->application
            )
        );

        gtk_window_set_default_size(
            GTK_WINDOW(
                this->gtk
            ),
            Browser::WIDTH,
            Browser::HEIGHT
        );

        // Init requirements
        this->header = new browser::Header(
            this
        );

        gtk_window_set_titlebar(
            GTK_WINDOW(
                this->gtk
            ),
            this->header->gtk
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}
