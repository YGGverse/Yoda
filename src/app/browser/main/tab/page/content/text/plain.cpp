#include "plain.hpp"

using namespace app::browser::main::tab::page::content::text;

Plain::Plain(
    const Glib::ustring & TEXT
) {
    set_wrap(
        true
    );

    set_selectable(
        true
    );

    set_text(
        TEXT
    );
}