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

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv,
        db,
        app
    );
}