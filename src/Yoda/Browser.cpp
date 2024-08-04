#include "Browser.h"

YodaBrowser::YodaBrowser(
    GtkApplication *application
) {
    this->gtk = gtk_application_window_new(
        application
    );

    gtk_window_set_title(
        GTK_WINDOW(
            this->gtk
        ),
        YodaBrowser::TITLE
    );

    gtk_window_set_default_size(
        GTK_WINDOW(
            this->gtk
        ),
        YodaBrowser::WIDTH,
        YodaBrowser::HEIGHT
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