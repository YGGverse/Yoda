#include "subtitle.hpp"

using namespace app::browser::header::main;

Subtitle::Subtitle()
{
    add_css_class(
        "subtitle"
    );

    set_single_line_mode(
        true
    );

    set_ellipsize(
        Pango::EllipsizeMode::END
    );

    set_valign(
        Gtk::Align::CENTER
    );

    set_width_chars(
        WIDTH_CHARS
    );

    hide();
}

Subtitle::~Subtitle() = default;

void Subtitle::set(
    const Glib::ustring & TEXT
) {

    set_text(
        TEXT
    );

    if (get_text().empty()) hide(); else show();
}