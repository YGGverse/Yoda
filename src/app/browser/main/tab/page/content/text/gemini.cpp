#include "gemini.hpp"
#include "gemini/markup.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & GEMTEXT
) : Gtk::Viewport( // add scrolled window features to childs
    NULL,
    NULL
) {
    // Init widget
    set_scroll_to_focus(
        false
    );

    set_child(
        * Gtk::make_managed<gemini::Markup>(
            GEMTEXT
        )
    );
}