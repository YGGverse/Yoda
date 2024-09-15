#include "content.hpp"
#include "content/text.hpp"

using namespace app::browser::main::tab::page;

Content::Content(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__OPEN_LINK_VARIANT
) {
    // Init actions
    action__open_link_variant = ACTION__OPEN_LINK_VARIANT;

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

// Getters
Glib::ustring Content::get_title()
{
    return title;
}

// Setters
void Content::update(
    const MIME & MIME,
    const Glib::ustring & SOURCE,
    GUri * uri
) {
    // Cleanup, free memory
    if (contentText != nullptr)
    {
        title.clear();

        remove(
            * contentText
        );

        delete contentText;

        contentText = nullptr;
    } // @TODO other types..

    // Create new DATA widget for MIME type requested
    switch (MIME)
    {
        case MIME::TEXT_GEMINI:

            contentText = new content::Text(
                action__open_link_variant,
                content::Text::Type::GEMINI,
                SOURCE,
                uri
            );

            title = contentText->get_title();

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