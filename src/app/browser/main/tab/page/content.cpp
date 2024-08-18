#include "content.hpp"
#include "content/text/gemini.hpp"
#include "content/text/plain.hpp"

using namespace app::browser::main::tab::page;

Content::Content()
{
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        true
    );

    widget = nullptr;
}

Content::~Content()
{
    delete widget;
};

// Public actions
void Content::text_gemini(
    const Glib::ustring & gemtext
) {
    update(
        new content::text::Gemini(
            gemtext
        )
    );
}

void Content::text_plain(
    const Glib::ustring & text
) {
    update(
        new content::text::Plain(
            text
        )
    );
}

// @TODO text_plain, picture, video, etc.

// Private helpers
void Content::update(
    Gtk::Widget * new_widget
) {
    if (widget != nullptr)
    {
        remove(
            * widget
        );

        delete widget;
    }

    widget = new_widget;

    append(
        * widget
    );
}