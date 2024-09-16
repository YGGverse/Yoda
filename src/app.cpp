#include "app.hpp"
#include "app/browser.hpp"

int main(
    int argc,
    char * argv[]
) {
    // Init database
    sqlite3 * database;

    sqlite3_open(
        "app.sqlite3",
        &database
    );

    // Init application
    auto app = Gtk::Application::create(
        "io.github.yggverse.Yoda.app"
    );

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv,
        database
    );
}