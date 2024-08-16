#ifndef APP_BROWSER_MAIN_TAB_LABEL_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_HPP

#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <gtkmm/gestureclick.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab
{
    class Label : public Gtk::Label
    {
        private:

            Glib::RefPtr<Gtk::GestureClick> controller;

            void on_click(
                int n,
                double x,
                double y
            );

        public:

            Label();

            ~Label();
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_HPP