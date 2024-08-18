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
void Content::set_text_gemini(
    const Glib::ustring & gemtext
) {
    set_widget(
        new content::text::Gemini(
            gemtext
        )
    );
}

void Content::set_text_plain(
    const Glib::ustring & text
) {
    set_widget(
        new content::text::Plain(
            text
        )
    );
}

// @TODO text_plain, picture, video, etc.

// Private helpers
void Content::set_widget(
    Gtk::Widget * object
) {
    if (widget != nullptr)
    {
        remove(
            * widget
        );

        delete widget;
    }

    widget = object;

    append(
        * widget
    );
}