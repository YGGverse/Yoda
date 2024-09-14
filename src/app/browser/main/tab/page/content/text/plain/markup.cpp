#include "markup.hpp"

using namespace app::browser::main::tab::page::content::text::plain;

Markup::Markup(
    const Glib::ustring & TEXT
) {
    // Init widget
    set_valign(
        Gtk::Align::START
    );

    set_wrap(
        true
    );

    set_selectable(
        true
    );

    set_use_markup(
        false // @TODO
    );

    set_text(
        TEXT
    );
}