#include "main.hpp"
#include "app/browser.hpp"
#include "lib/database.hpp"

int main(
    int argc,
    char * argv[]
) {
    // Init profile database
    const std::shared_ptr<lib::Database> db(
        new lib::Database(
            "database.sqlite3"
        )
    );

    // Init app
    const Glib::RefPtr<Gtk::Application> app = Gtk::Application::create(
        "io.github.yggverse.Yoda"
    );

    app->add_action(
        "quit",
        [app]
        {
            app->quit();
        }
    );

    // Init accels @TODO db settings
    app->set_accel_for_action(
        "win.main_tab_append",
        "<Primary>t"
    );

    app->set_accel_for_action(
        "win.main_tab_update",
        "<Primary>r"
    );

    app->set_accel_for_action(
        "win.debug",
        "<Primary>i"
    );

    app->set_accel_for_action(
        "app.quit",
        "<Primary>q"
    );

    // Launch browser component
    return app->make_window_and_run<app::Browser>(
        argc,
        argv
        //app,
        //db
    );
}