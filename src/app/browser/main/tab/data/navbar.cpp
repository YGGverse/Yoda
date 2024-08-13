#include "navbar.hpp"

using namespace app::browser::main::tab::data;

Navbar::Navbar()
{
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_homogeneous(
        true
    );
}

Navbar::~Navbar() = default;
