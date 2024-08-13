#include "navbar.hpp"
#include "navbar/base.hpp"
#include "navbar/update.hpp"

using namespace app::browser::main::tab::data;

Navbar::Navbar()
{
    // Init container
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_spacing(
        SPACING
    );

    set_margin_top(
        MARGIN
    );

    set_margin_start(
        MARGIN
    );

    set_margin_end(
        MARGIN
    );

    set_margin_bottom(
        MARGIN
    );

    // Init elements
    base = new navbar::Base();

        append(
            * base
        );

    update = new navbar::Update();

        append(
            * update
        );
}

Navbar::~Navbar() = default;
