#include "gemini.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & GEMTEXT
) : Gtk::Viewport( // add scrolled window features support
    NULL,
    NULL
) {
    set_scroll_to_focus(
        false
    );

    auto label = Gtk::make_managed<Gtk::Label>( // @TODO separated file?
        GEMTEXT
    );

        // Init widget
        label->set_valign(
            Gtk::Align::START
        );

        label->set_wrap(
            true
        );

        label->set_selectable(
            true
        );

        label->set_use_markup(
            true
        );

    set_child(
        * label
    );
}