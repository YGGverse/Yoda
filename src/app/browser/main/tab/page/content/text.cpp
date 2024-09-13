#include "text.hpp"
#include "text/gemini.hpp"
// #include "text/plain.hpp" @TODO

using namespace app::browser::main::tab::page::content;

Text::Text(
    const Type & TYPE,
    const Glib::ustring & VALUE
) {
    switch (TYPE)
    {
        case GEMINI:

            set_child(
                * Gtk::make_managed<text::Gemini>(
                    VALUE
                )
            );

        break;

        case PLAIN:

            // @TODO

        break;

        default:

            throw _("Invalid text type enum"); // @TODO
    }
}