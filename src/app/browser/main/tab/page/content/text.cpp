#include "text.hpp"
#include "text/gemini.hpp"
#include "text/plain.hpp"

using namespace app::browser::main::tab::page::content;

Text::Text(
    const Type & TYPE,
    const Glib::ustring & VALUE
) {
    // Init components
    textGemini = nullptr;
    textPlain = nullptr;

    // GtkLabel does not support ScrolledWindow features, create GtkViewport
    auto viewport = new Gtk::Viewport( // @TODO manage
        NULL, //Gtk::Adjustment::H
        NULL  //Gtk::Adjustment::V
    ); // @TODO manage, optimize

    viewport->set_scroll_to_focus(
        false
    );

    // Detect text driver by text type requested
    switch (TYPE)
    {
        case GEMINI:

            textGemini = new text::Gemini(
                VALUE
            );

            viewport->set_child(
                * textGemini
            );

        break;

        case PLAIN:

            // @TODO

        break;

        default:

            throw _("Invalid text type enum"); // @TODO
    }

    set_child(
        * viewport
    );
}

Text::~Text()
{
    delete textGemini;
    delete textPlain;

    // @TODO
}