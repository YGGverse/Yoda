#include "gemini.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & gemtext
) {
    set_wrap(
        true
    );

    set_selectable(
        true
    );

    set_markup(
        gemtext // @TODO
    );
}

Gemini::~Gemini() = default;