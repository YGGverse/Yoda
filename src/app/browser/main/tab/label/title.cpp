#include "title.hpp"

using namespace app::browser::main::tab::label;

Title::Title()
{
    set_text(
        _("New page")
    );

    set_ellipsize(
        Pango::EllipsizeMode::END
    );

    set_width_chars(
        WIDTH_CHARS
    );

    set_single_line_mode(
        true
    );
}