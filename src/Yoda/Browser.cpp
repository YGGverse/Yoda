#include "Browser.h"

Yoda::Browser::Browser(
    GtkApplication *application
) {
    this->gtk = gtk_application_window_new(
        GTK_APPLICATION(
            application
        )
    );

    gtk_window_set_title(
        GTK_WINDOW(
            this->gtk
        ),
        Browser::TITLE
    );

    gtk_window_set_default_size(
        GTK_WINDOW(
            this->gtk
        ),
        Browser::WIDTH,
        Browser::HEIGHT
    );

    GtkWidget *label = gtk_label_new(
        "Hello, World!"
    );

    gtk_window_set_child(
        GTK_WINDOW(
            this->gtk
        ),
        label
    );

    gtk_widget_show(
        GTK_WIDGET(
            this->gtk
        )
    );

    // @TODO signals
}