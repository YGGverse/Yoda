#include "debug.h"

namespace app::browser::header::menu::main
{
    // Construct
    Debug::Debug(
        Main *main
    ) {
        // Init dependencies
        this->main = main;

        // Init action object
        this->action = g_simple_action_new(
            Debug::ACTION_ID,
            NULL
        );

        g_action_map_add_action(
            G_ACTION_MAP(
                this->main->menu->header->browser->app
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
            Debug::ACTION_NS,
            Debug::ACTION_ID
        );

        // Init keyboard accelerators
        // https://docs.gtk.org/gtk4/func.accelerator_parse.html
        const gchar *accels[] = {
            Debug::ACCEL_1,  // First accelerator
            Debug::ACCEL_2,  // Second accelerator
            NULL
        };

        gtk_application_set_accels_for_action(
            GTK_APPLICATION(
                this->main->menu->header->browser->app
            ),
            action,
            accels
        );

        // Init menu item object
        this->item = g_menu_item_new(
            Debug::LABEL,
            action
        );

        // Connect events
        g_signal_connect(
            G_SIMPLE_ACTION(
                this->action
            ),
            "activate",
            G_CALLBACK(
                Debug::_activate
            ),
            NULL
        );
    }

    // Events
    void Debug::_activate(
        GSimpleAction* action,
        GVariant* parameter,
        gpointer user_data
    ) {
        gtk_window_set_interactive_debugging(
            true
        );
    }
}
