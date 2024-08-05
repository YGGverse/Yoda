#include "tab.h"

namespace app
{
    namespace browser
    {
        namespace container
        {
            /**
             * Construct
             */
            Tab::Tab(
                Container *container
            ) {
                // Init dependencies
                this->container = container;

                // Init GTK
                this->gtk = gtk_notebook_new();

                gtk_notebook_set_scrollable(
                    GTK_NOTEBOOK(
                        this->gtk
                    ),
                    Tab::SCROLLABLE
                );

                gtk_widget_show(
                    GTK_WIDGET(
                        this->gtk
                    )
                );
            }

            /**
             * Append new tab
             */
            void Tab::append(
                char *request,
                bool open = true,
                bool focus = true
            ) {

                // Init new page
                Page *page = new Page(
                    this->container
                );

                gtk_notebook_append_page(
                    GTK_NOTEBOOK(
                        this->gtk
                    ),
                    GTK_WIDGET(
                        page->gtk
                    ),
                    NULL // @TODO label
                );

                gtk_notebook_set_tab_reorderable(
                    GTK_NOTEBOOK(
                        this->gtk
                    ),
                    GTK_WIDGET(
                        page->gtk
                    ),
                    Tab::REORDERABLE
                );

                // Page open requested
                if (open)
                {
                    page->open(
                        request,
                        false // history
                    );
                }

                else
                {
                    page->init(
                        request,
                        true // focus @TODO boolean empty(request)
                    );
                }

                // Focus requested
                if (focus)
                {
                    gtk_notebook_set_current_page(
                        GTK_NOTEBOOK(
                            this->gtk
                        ),
                        gtk_notebook_page_num(
                            GTK_NOTEBOOK(
                                this->gtk
                            ),
                            GTK_WIDGET(
                                page->gtk
                            )
                        )
                    );
                }

                // Render
                gtk_widget_show(
                    GTK_WIDGET(
                        this->gtk
                    )
                );
            }
        }
    }
}
