#include "main.h"

void activate(
    GtkApplication *application
) {
    // Init default component
    new Yoda::Browser(
        application
    );
}

int main(
    int argc,
    char *argv[]
) {
    // Create a new application
    GtkApplication *application = gtk_application_new(
        NULL,
        G_APPLICATION_DEFAULT_FLAGS
    );

    // Connect the "activate" signal to the callback function
    g_signal_connect(
        application,
        "activate",
        G_CALLBACK(
            activate
        ),
        NULL
    );

    // Run the application
    int status = g_application_run(
        G_APPLICATION(
            application
        ),
        argc,
        argv
    );

    // Clean up
    g_object_unref(
        application
    );

    // Result
    return status;
}