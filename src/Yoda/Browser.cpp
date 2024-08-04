#include "Browser.h"

Yoda::Browser::Browser(
    GtkApplication *application
) {
    this->gtk = gtk_application_window_new(
        GTK_APPLICATION(
            application
        )
    );

    gtk_window_set_default_size(
        GTK_WINDOW(
            this->gtk
        ),
        Browser::WIDTH,
        Browser::HEIGHT
    );

    gtk_window_set_titlebar(
        GTK_WINDOW(
            this->gtk
        ),
        (new YodaBrowser::Header())->gtk
    );

    gtk_widget_show(
        GTK_WIDGET(
            this->gtk
        )
    );
}