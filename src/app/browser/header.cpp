#include "header.h"

namespace app
{
    namespace browser
    {
        // Construct
        Header::Header(
            Browser *browser
        ) {
            // Init dependencies
            this->browser = browser;

            // Init GTK
            this->gtk = gtk_header_bar_new();

            gtk_header_bar_set_show_title_buttons(
                GTK_HEADER_BAR(
                    this->gtk
                ),
                Header::SHOW_TITLE_BUTTONS
            );

            // Init title widget
            gtk_header_bar_set_title_widget(
                GTK_HEADER_BAR(
                    this->gtk
                ),
                (new Menu(this->browser))->gtk // @TODO
            );

            // Render
            gtk_widget_show(
                GTK_WIDGET(
                    this->gtk
                )
            );
        }
    }
}
