#include "text.hpp"
#include "text/gemini.hpp"
#include "text/plain.hpp"

using namespace app::browser::main::tab::page::content;

Text::Text(
    const Type & TYPE,
    const Glib::ustring & TEXT
) {
    switch (TYPE)
    {
        case GEMINI:

            set_child(
                * Gtk::make_managed<text::Gemini>(
                    TEXT,
                    title
                )
            );

        break;

        case PLAIN:

            set_child(
                * Gtk::make_managed<text::Plain>(
                    TEXT
                )
            );

        break;

        default:

            throw _("Invalid text type enum"); // @TODO
    }
}

// Getters
Glib::ustring Text::get_title()
{
    return title;
}