#include "content.hpp"
#include "content/text.hpp"

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
    const Glib::ustring & GEMTEXT
) {
    auto contentText = new content::Text; // @TODO manage

    contentText->set_gemini(
        GEMTEXT
    );

    set_widget(
        contentText
    );
}

void Content::set_text_plain(
    const Glib::ustring & TEXT
) {
    // @TODO
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