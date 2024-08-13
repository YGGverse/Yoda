#include "data.hpp"
#include "data/navbar.hpp"

using namespace app::browser::main::tab;

Data::Data()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        true
    );

    // Init elements
    navbar = new data::Navbar();

    append(
        * navbar
    );
}

Data::~Data() = default;
