#include "header.h"

namespace app
{
    namespace browser
    {
        // Construct
        Menu::Menu(
            Browser *browser
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
}
