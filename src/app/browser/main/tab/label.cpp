#include "label.hpp"

using namespace app::browser::main::tab;

Label::Label()
{
    set_text(
        _("New tab")
    );

    // Setup label controller
    controller = Gtk::GestureClick::create();

        /* @TODO remove as default
        controller->set_button(
            GDK_BUTTON_PRIMARY
        );*/

        controller->signal_pressed().connect(
            sigc::mem_fun(
                * this,
                & Label::on_click
            )
        );

        add_controller(
            controller
        );
}

Label::~Label() = default;

void Label::on_click(
    int n,
    double x,
    double y
) {
    if (n == 2) // double click
    {
        //Tab::close(
        //    -1 // active
        //);
    }
}
