#ifndef APP_BROWSER_MAIN_TAB_H
#define APP_BROWSER_MAIN_TAB_H

#include <glibmm/i18n.h>
#include <gtkmm/gestureclick.h>
#include <gtkmm/label.h>
#include <gtkmm/notebook.h>

namespace app::browser::main
{
    class Tab : public Gtk::Notebook
    {
        private:

            void on_label_click(
                int n,
                double x,
                double y
            );

        public:

            const bool SCROLLABLE = true;
            const bool REORDERABLE = true;

            const char * LABEL = _("New tab");

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

            Tab();

            ~Tab();

            void append(
                const char * request,
                bool open,
                bool focus
            );

            void close();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_H