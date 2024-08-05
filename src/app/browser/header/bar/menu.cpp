#include "menu.h"

namespace app::browser::header::bar
{
    // Construct
    Menu::Menu(
        Bar *bar
    ) {
        // Init dependencies
        this->bar = bar;

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
