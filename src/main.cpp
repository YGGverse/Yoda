#include "main.h"

int main(
    int argc,
    char* argv[]
) {
    // Init app
    auto app = Gtk::Application::create(
        APPLICATION_ID
    );

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv
    );
}