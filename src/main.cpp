#include "main.hpp"
#include "app/browser.hpp"

int main(
    int argc,
    char * argv[]
) {
    // Init app
    auto app = Gtk::Application::create(
        "io.github.yggverse.Yoda"
    );

    // Init actions
    app->add_action(
        "quit",
        sigc::mem_fun(
            * app,
            & Gtk::Application::quit
        )
    );

    // Init accels
    app->set_accel_for_action(
        "app.quit",
        "<Primary>q"
    );

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv,
        app
    );
}