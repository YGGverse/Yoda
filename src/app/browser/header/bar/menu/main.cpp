#include "main.h"

namespace app::browser::header::bar::menu
{
    // Construct
    Main::Main(
        Menu *menu
    ) {
        // Init dependencies
        this->menu = menu;

        // Init GTK
        this->gtk = gtk_popover_new();
    }
}
