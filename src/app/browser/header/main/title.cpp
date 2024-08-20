#include "title.hpp"

using namespace app::browser::header::main;

Title::Title()
{
    add_css_class(
        "title"
    );

    set_single_line_mode(
        true
    );

    set_ellipsize(
        Pango::EllipsizeMode::END
    );

    set_width_chars(
        WIDTH_CHARS
    );

    // @TODO
}

Title::~Title() = default;