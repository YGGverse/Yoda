#include "content.hpp"

using namespace app::browser::main::tab::page;

Content::Content()
{
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        true
    );
}

Content::~Content() = default;

void Content::set(
    const Glib::ustring & buffer
) {
    // @TODO
}
