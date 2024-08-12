#include "tab.hpp"

using namespace app::browser::main;

Tab::Tab()
{
    set_scrollable(
        true
    );
}

Tab::~Tab() = default;

void Tab::append(
    const char * request,
    bool open,
    bool focus
) {
    // Init new tab label
    Gtk::Label * name = new Gtk::Label(
        _("New tab")
    );

    // Setup label controller
    auto controller = Gtk::GestureClick::create();

        /* @TODO remove as default
        controller->set_button(
            GDK_BUTTON_PRIMARY
        );*/

        controller->signal_pressed().connect(
            sigc::mem_fun(
                * this,
                & Tab::on_label_click
            )
        );

        name->add_controller(
            controller
        );

    // Init tab data container @TODO
    Gtk::Label * data = new Gtk::Label("data");

    append_page(
        * data,
        * name
    );

    set_tab_reorderable(
        * data,
        true
    );

    if (focus)
    {
        set_current_page(
            page_num(
                * data
            )
        );
    }
};

void Tab::close(
    int number
) {
    remove_page(
        number
    );

    // @TODO clean memory
    // @TODO fix GtkGizmo reported min height, but sizes must be >= 0
}

void Tab::close_left()
{} // @TODO

void Tab::close_right()
{} // @TODO

void Tab::close_all()
{
    while (0 <= get_current_page())
    {
        close(
            -1 // active
        );
    }
}

void Tab::on_label_click(
    int n,
    double x,
    double y
) {
    if (n == 2) // double click
    {
        close(
            -1 // active
        );
    }
}