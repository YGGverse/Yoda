#include "app.hpp"
#include "app/browser.hpp"

int main(
    int argc,
    char * argv[]
) {
    // Init database
    sqlite3 * db;

    sqlite3_open(
        "database.sqlite3",
        &db
    );

    // Init application
    auto app = Gtk::Application::create(
        "io.github.yggverse.Yoda"
    );

        // Init actions
        app->add_action(
            "quit",
            [app]
            {
                app->quit();
            }
        );

            app->set_accel_for_action(
                "app.quit",
                "<Primary>q"
            );

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv,
        db,
        app
    );
}