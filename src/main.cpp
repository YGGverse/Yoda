#include "main.hpp"
#include "app/browser.hpp"
#include "lib/database.hpp"

int main(
    int argc,
    char * argv[]
) {
    // Init profile database
    const std::shared_ptr<lib::Database> DB( // @TODO
        new lib::Database(
            "database.sqlite3"
        )
    );

    // Init app
    const Glib::RefPtr<Gtk::Application> APP = Gtk::Application::create(
        "io.github.yggverse.Yoda"
    );

    APP->add_action(
        "quit",
        [APP]
        {
            APP->quit();
        }
    );

        APP->set_accel_for_action(
            "app.quit",
            "<Primary>q"
        );

    // Launch browser component
    return APP->make_window_and_run<app::Browser>(
        argc,
        argv,
        APP
        // DB
    );
}