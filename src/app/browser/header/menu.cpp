#include "menu.h"

namespace app::browser::header
{
    // Construct
    Menu::Menu(
        Header *header
    ) {
        // Init dependencies
        this->header = header;

        // Init GTK
        this->gtk = gtk_menu_button_new();

        // Init requirements
        this->main = new menu::Main(
            this
        );

        // Init main popover
        gtk_menu_button_set_popover(
            GTK_MENU_BUTTON(
                this->gtk
            ),
            GTK_WIDGET(
                this->main->gtk
            )
        );

        // Render
        gtk_widget_show(
            GTK_WIDGET(
                this->gtk
            )
        );
    }
}
