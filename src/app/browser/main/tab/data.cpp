#include "data.hpp"
#include "data/navbar.hpp"

using namespace app::browser::main::tab;

Data::Data()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init children components
    navbar = new data::Navbar();

    append(
        * navbar
    );

    // Init actions group
    action_group = Gio::SimpleActionGroup::create();

    // Define actions
    action_group->add_action(
        "update",
        sigc::mem_fun(
            * this,
            & Data::update
        )
    );

    insert_action_group(
        "tab",
        action_group
    );
}

Data::~Data() = default;

void Data::update()
{} // @TODO
