#include "plain.hpp"
#include "plain/reader.hpp"

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

    set_child(
        * Gtk::make_managed<plain::Reader>(
            TEXT
        )
    );
}