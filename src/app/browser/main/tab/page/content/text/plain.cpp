#include "plain.hpp"

using namespace app::browser::main::tab::page::content::text;

Plain::Plain(
    const Glib::ustring & TEXT
) : Gtk::Viewport( // add scrolled window features support
    NULL,
    NULL
) {
    set_scroll_to_focus(
        false
    );

    auto label = Gtk::make_managed<Gtk::Label>( // @TODO separated file?
        TEXT
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
            false
        );

    set_child(
        * label
    );
}