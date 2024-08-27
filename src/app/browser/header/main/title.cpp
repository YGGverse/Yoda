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

    set_valign(
        Gtk::Align::CENTER
    );

    set_width_chars(
        WIDTH_CHARS
    );

    set(
        DEFAULT_TEXT
    );
}

Title::~Title() = default;

void Title::set(
    const Glib::ustring & TEXT
) {
    // Append app name postfix
    set_text(
        TEXT.empty() || TEXT == DEFAULT_TEXT ? DEFAULT_TEXT : TEXT + " - " + DEFAULT_TEXT
    );

    // Use GTK3 header behaviour
    if (get_text().empty()) hide(); else show();
}