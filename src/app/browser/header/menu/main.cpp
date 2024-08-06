#include "main.h"

namespace app::browser::header::menu
{
    // Construct
    Main::Main(
        Menu *menu
    ) {
        // Init dependencies
        this->menu = menu;

        // Init model
        this->model = g_menu_new();

        // Init tab submenu
        this->tab = new main::Tab(
            this
        );

        g_menu_append_submenu(
            G_MENU(
                this->model
            ),
            this->tab->LABEL,
            G_MENU_MODEL(
                this->tab->model
            )
        );

        // Init debug menu
        this->debug = new main::Debug(
            this
        );

        g_menu_append_item(
            G_MENU(
                this->model
            ),
            G_MENU_ITEM(
                this->debug->item
            )
        );

        // Init quit menu
        this->quit = new main::Quit(
            this
        );

        g_menu_append_item(
            G_MENU(
                this->model
            ),
            G_MENU_ITEM(
                this->quit->item
            )
        );

        // Create a new GtkPopoverMenu from the GMenuModel
        this->gtk = gtk_popover_menu_new_from_model(
            G_MENU_MODEL(
                this->model
            )
        );
    }
}
