#include "data.hpp"
#include "data/navbar.hpp"
#include "data/content.hpp"

using namespace app::browser::main::tab;

Data::Data()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init components
    navbar = new data::Navbar();

        append(
            * navbar
        );

    content = new data::Content();

        append(
            * content
        );

    // Init actions group
    action_group = Gio::SimpleActionGroup::create();

        // Define group actions
        action_group->add_action(
            "update",
            sigc::mem_fun(
                * this,
                & Data::update
            )
        );

    insert_action_group(
        "data",
        action_group
    );
}

Data::~Data() = default;

// Actions
void Data::update()
{
    // navbar->get_request_value() @TODO
}
