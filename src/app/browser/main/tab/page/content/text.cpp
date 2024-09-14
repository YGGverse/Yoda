#include "text.hpp"
#include "text/gemini.hpp"
#include "text/plain.hpp"

using namespace app::browser::main::tab::page::content;

Text::Text(
    const Type & TYPE,
    const Glib::ustring & SOURCE,
    GUri * uri
) {
    switch (TYPE)
    {
        case GEMINI:

            set_child(
                * Gtk::make_managed<text::Gemini>(
                    SOURCE,
                    title,
                    uri
                )
            );

        break;

        case PLAIN:

            set_child(
                * Gtk::make_managed<text::Plain>(
                    SOURCE
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