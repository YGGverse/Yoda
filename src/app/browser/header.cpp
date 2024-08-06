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

            // Init menu element
            this->menu = new header::Menu(
                this
            );

            gtk_header_bar_pack_start(
                GTK_HEADER_BAR(
                    this->gtk
                ),
                this->menu->gtk
            );

            // Init new tab element
            this->tab = new header::Tab(
                this
            );

            gtk_header_bar_pack_start(
                GTK_HEADER_BAR(
                    this->gtk
                ),
                this->tab->gtk
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
