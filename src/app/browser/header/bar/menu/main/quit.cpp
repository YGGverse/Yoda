#include "quit.h"

namespace app::browser::header::bar::menu::main
{
    // Construct
    Quit::Quit(
        Main *main
    ) {
        // Init dependencies
        this->main = main;

        // Init action object
        this->action = g_simple_action_new(
            Quit::ACTION_ID,
            NULL
        );

        g_action_map_add_action(
            G_ACTION_MAP(
                this->main->menu->bar->header->browser->application
            ),
            G_ACTION(
                this->action
            )
        );

        // Init action NS
        gchar action[255];

        g_snprintf(
            action,
            sizeof(
                action
            ),
            Quit::ACTION_NS,
            Quit::ACTION_ID
        );

        // Init keyboard accelerators
        // https://docs.gtk.org/gtk4/func.accelerator_parse.html
        const gchar *accels[] = {
            Quit::ACCEL_1,  // First accelerator
            Quit::ACCEL_2,  // Second accelerator
            NULL
        };

        gtk_application_set_accels_for_action(
            GTK_APPLICATION(
                this->main->menu->bar->header->browser->application
            ),
            action,
            accels
        );

        // Init menu item object
        this->item = g_menu_item_new(
            Quit::LABEL,
            action
        );

        // Connect events
        g_signal_connect(
            G_SIMPLE_ACTION(
                this->action
            ),
            "activate",
            G_CALLBACK(
                Quit::_activate
            ),
            G_APPLICATION(
                this->main->menu->bar->header->browser->application
            )
        );
    }

    // Events
    void Quit::_activate(
        GSimpleAction* action,
        GVariant* parameter,
        gpointer user_data
    ) {
        g_application_quit(
            G_APPLICATION(
                user_data
            )
        );
    }
}
