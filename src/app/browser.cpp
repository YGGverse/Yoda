#include "browser.h"

namespace app
{
    // Construct
    Browser::Browser(
        GtkApplication *application
    ) {
        // Init dependencies
        this->app = application;

        // Init GTK
        this->gtk = gtk_application_window_new(
            GTK_APPLICATION(
                this->app
            )
        );

        gtk_window_set_default_size(
            GTK_WINDOW(
                this->gtk
            ),
            Browser::WIDTH,
            Browser::HEIGHT
        );

        // Init components
        this->header = new browser::Header(
            this
        );

        gtk_window_set_titlebar(
            GTK_WINDOW(
                this->gtk
            ),
            GTK_WIDGET(
                this->header->gtk
            )
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );

        // Connect signals
        g_signal_connect(
            G_APPLICATION(
                this->app
            ),
            "shutdown",
            G_CALLBACK(
                _shutdown
            ),
            NULL
        );
    }

    // Events
    void Browser::_shutdown(
        GtkApplication *application
    ) {
        // @TODO save session, clean cache, etc
        g_print("Shutdown..\n");
    }
}
