#include "content.hpp"
#include "content/text.hpp"

using namespace app::browser::main::tab::page;

Content::Content()
{
    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        true
    );

    set_hexpand(
        true
    );

    set_vexpand(
        true
    );

    // Init child types
    contentText = nullptr;
}

Content::~Content()
{
    delete contentText;
}

// Setters
void Content::update(
    const MIME & MIME,
    const Glib::ustring & DATA
) {
    // Cleanup, free memory
    if (contentText != nullptr)
    {
        remove(
            * contentText
        );

        delete contentText;

        contentText = nullptr;
    } // @TODO other types..

    // Create new content widget for MIME type requested
    switch (MIME)
    {
        case MIME::TEXT_GEMINI:

            contentText = new content::Text(
                content::Text::Type::GEMINI,
                DATA
            );

            append(
                * contentText
            );

        break;

        case MIME::TEXT_PLAIN:

            // @TODO

        break;

        default:

            throw _("Invalid content MIME type");
    }
}