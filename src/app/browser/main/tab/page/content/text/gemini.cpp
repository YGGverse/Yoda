#include "gemini.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & GEMTEXT
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
        true
    );

    set_markup(
        GEMTEXT//markup
    );
}