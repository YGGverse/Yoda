#include "main.h"

namespace app::browser::header::bar::menu
{
    // Construct
    Main::Main(
        Menu *menu
    ) {
        // Init dependencies
        this->menu = menu;

        // Init GMenu Model
        this->_model = g_menu_new();

        // Init menu items
        this->quit = new main::Quit(
            this
        );

        g_menu_append_item(
            G_MENU(
                this->_model
            ),
            G_MENU_ITEM(
                this->quit->item
            )
        );

        // Create a new GtkPopoverMenu from the GMenuModel
        this->gtk = gtk_popover_menu_new_from_model(
            G_MENU_MODEL(
                this->_model
            )
        );
    }
}
