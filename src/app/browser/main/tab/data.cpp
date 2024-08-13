#include "data.hpp"

using namespace app::browser::main::tab;

Data::Data()
{
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        true
    );
}

Data::~Data() = default;
