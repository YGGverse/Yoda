#include "gemini.hpp"
#include "gemini/reader.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & GEMTEXT,
    Glib::ustring & title
) : Gtk::Viewport( // add scrolled window features to childs
    NULL,
    NULL
) {
    // Init components
    auto geminiReader = Gtk::make_managed<gemini::Reader>(
        GEMTEXT
    );

        // Grab title
        title = geminiReader->get_title();

    // Init widget
    set_scroll_to_focus(
        false
    );

    set_child(
        * geminiReader
    );
}