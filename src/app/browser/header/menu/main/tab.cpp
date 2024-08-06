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

        // Init new tab menu
        this->append = new tab::Append(
            this
        );

        g_menu_append_item(
            G_MENU(
                this->model
            ),
            G_MENU_ITEM(
                this->append->item
            )
        );

        // Init menu
        this->gtk = gtk_popover_menu_new_from_model(
            G_MENU_MODEL(
                this->model
            )
        );
    }
}
