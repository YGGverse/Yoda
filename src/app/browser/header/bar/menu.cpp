#include "menu.h"

namespace app::browser::header::bar
{
    // Construct
    Menu::Menu(
        Bar *bar
    ) {
        // Init GTK
        this->gtk = gtk_menu_button_new();

        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}
