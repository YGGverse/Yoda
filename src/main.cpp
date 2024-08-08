#include "main.h"

int main(
    int argc,
    char* argv[]
) {
    auto app = Gtk::Application::create(
        APPLICATION_ID
    );

    return app->make_window_and_run<app::Browser>(
        argc,
        argv
    );
}