#include "tab.h"

namespace app::browser::header::menu::main
{
    // Construct
    Tab::Tab(
        Main *main
    ) {
        // Init dependencies
        this->main = main;

        // Init model
        this->model = g_menu_new();

        // Init menu
        this->gtk = gtk_popover_menu_new_from_model(
            G_MENU_MODEL(
                this->model
            )
        );
    }
}
