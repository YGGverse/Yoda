#include "content.hpp"

using namespace app::browser::main::tab::page;
using namespace std;

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
    string buffer
) {
    // @TODO
}
