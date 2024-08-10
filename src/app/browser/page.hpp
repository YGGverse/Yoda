#ifndef APP_BROWSER_PAGE_H
#define APP_BROWSER_PAGE_H

#include <gtkmm/notebook.h>

namespace app::browser
{
    class Page : public Gtk::Notebook
    {
        public:

            const bool SCROLLABLE = true;
            const bool REORDERABLE = true;

            class Navbar
            {
                public:

                    Navbar();
            };

            class Body
            {
                public:

                    Body();
            };

            Page();

            void append(
                char* request,
                bool open = true,
                bool focus = false
            );

            void update();
    };
}

#endif // APP_BROWSER_PAGE_H