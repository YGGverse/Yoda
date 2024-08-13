#include "content.hpp"

using namespace app::browser::main::tab::data;

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
