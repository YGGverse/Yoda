#include "text.hpp"
#include "text/gemini.hpp"
#include "text/plain.hpp"

using namespace app::browser::main::tab::page::content;

Text::Text()
{
    // @TODO GtkViewport?
}

void Text::set_gemini(
    const Glib::ustring & GEMTEXT
) {
    auto viewport = new Gtk::Viewport( // @TODO
        NULL, //Gtk::Adjustment::H
        NULL  //Gtk::Adjustment::V
    );

    viewport->set_scroll_to_focus(
        false
    );

    viewport->set_child(
        * new text::Gemini( // @TODO manage
            GEMTEXT
        )
    );

    set_child(
        * viewport
    );
}

void Text::set_plain(
    const Glib::ustring & TEXT
) {
    // @TODO
}