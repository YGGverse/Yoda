#include "source.hpp"

using namespace app::browser::main::tab::page::content::text::gemini;

Source::Source(
    const Glib::ustring & GEMTEXT
) {
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
        Glib::ustring::sprintf(
            "<tt>%s</tt>",
            Glib::Markup::escape_text(
                GEMTEXT
            )
        )
    );
}