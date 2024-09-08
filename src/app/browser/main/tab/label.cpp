#include "label.hpp"

using namespace app::browser::main::tab;

Label::Label(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__CLOSE_ACTIVE
) {
    // Init actions
    action__close_active = ACTION__CLOSE_ACTIVE;

    // Setup label controller
    auto GtkGestureClick = Gtk::GestureClick::create();

        /* @TODO remove as default
        controller->set_button(
            GDK_BUTTON_PRIMARY
        );*/

        GtkGestureClick->signal_pressed().connect(
            [this](int n, double x, double y)
            {
                if (n == 2) // double click
                {
                    action__close_active->activate();
                }
            }
        );

        add_controller(
            GtkGestureClick
        );
}