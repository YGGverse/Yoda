#include "plain.hpp"

using namespace app::browser::main::tab::page::content::text;

Plain::Plain(
    const Glib::ustring & text
) {
    set_wrap(
        true
    );

    set_selectable(
        true
    );

    set_text(
        text
    );
}

Plain::~Plain() = default;