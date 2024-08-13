#include "navbar.hpp"
#include "navbar/base.hpp"

using namespace app::browser::main::tab::data;

Navbar::Navbar()
{
    // Init container
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_homogeneous(
        true
    );

    // Init elements
    base = new navbar::Base();

    append(
        * base
    );
}

Navbar::~Navbar() = default;
