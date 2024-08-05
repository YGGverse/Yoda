#include "page.h"

namespace app
{
    namespace browser
    {
        namespace container
        {
            /**
             * Construct
             */
            Page::Page(
                Container *container
            ) {
                // Init GTK
                this->gtk = gtk_box_new(
                    GTK_ORIENTATION_VERTICAL,
                    Page::SPACING
                );

                gtk_widget_show(
                    GTK_WIDGET(
                        this->gtk
                    )
                );
            }

            /**
             * Init empty page
             */
            void Page::init(
                char *request,
                bool focus
            ) {
                // @TODO
            }

            /**
             * Open page request
             */
            void Page::open(
                char *request,
                bool history
            ) {
                // @TODO
            }
        }
    }
}