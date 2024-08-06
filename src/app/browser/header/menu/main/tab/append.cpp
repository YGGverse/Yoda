#include "append.h"

namespace app::browser::header::menu::main::tab
{
    // Construct
    Append::Append(
        Tab *tab
    ) {
        // Init dependencies
        this->tab = tab;

        // Init action object
        this->action = g_simple_action_new(
            Append::ACTION_ID,
            NULL
        );

        g_action_map_add_action(
            G_ACTION_MAP(
                this->tab->main->menu->header->browser->app
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
            Append::ACTION_NS,
            Append::ACTION_ID
        );

        // Init keyboard accelerators
        // https://docs.gtk.org/gtk4/func.accelerator_parse.html
        const gchar *accels[] = {
            Append::ACCEL_1,  // First accelerator
            Append::ACCEL_2,  // Second accelerator
            NULL
        };

        gtk_application_set_accels_for_action(
            GTK_APPLICATION(
                this->tab->main->menu->header->browser->app
            ),
            action,
            accels
        );

        // Init menu item object
        this->item = g_menu_item_new(
            Append::LABEL,
            action
        );

        // Connect events
        g_signal_connect(
            G_SIMPLE_ACTION(
                this->action
            ),
            "activate",
            G_CALLBACK(
                Append::_activate
            ),
            G_APPLICATION(
                this->tab->main->menu->header->browser->app
            )
        );
    }

    // Events
    void Append::_activate(
        GSimpleAction* action,
        GVariant* parameter,
        gpointer user_data
    ) {
        // @TODO
    }
}
