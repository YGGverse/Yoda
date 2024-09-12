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
    set_child(
        * new text::Gemini( // @TODO manage
            GEMTEXT
        )
    );
}

void Text::set_plain(
    const Glib::ustring & TEXT
) {
    // @TODO
}