#include "gemini.hpp"
#include "gemini/reader.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & REQUEST,
    const Glib::ustring & GEMTEXT,
    Glib::ustring & title
) : Gtk::Viewport( // add scrolled window features to childs
    NULL,
    NULL
) {
    // Init widget
    set_scroll_to_focus(
        false
    );

    set_child(
        * Gtk::make_managed<gemini::Reader>(
            REQUEST,
            GEMTEXT,
            title
        )
    );
}