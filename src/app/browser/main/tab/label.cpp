#include "label.hpp"

using namespace app::browser::main::tab;

Label::Label(
    const Glib::ustring & TEXT
) {
    set_text(
        TEXT
    );

    // Setup label controller
    auto GtkGestureClick_RefPtr = Gtk::GestureClick::create();

        /* @TODO remove as default
        controller->set_button(
            GDK_BUTTON_PRIMARY
        );*/

        GtkGestureClick_RefPtr->signal_pressed().connect(
            [this](int n, double x, double y)
            {
                if (n == 2) // double click
                {
                    activate_action(
                        "win.tab_close"
                    );
                }
            }
        );

        add_controller(
            GtkGestureClick_RefPtr
        );
}

Label::~Label() = default;