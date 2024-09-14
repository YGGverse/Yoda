#include "gemini.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::ustring & GEMTEXT
) : Gtk::Viewport( // add scrolled window features support
    NULL,
    NULL
) {
    set_scroll_to_focus(
        false
    );

    auto label = Gtk::make_managed<Gtk::Label>( // @TODO separated file?
        to_pango_markup(
            GEMTEXT
        )
    );

        // Init widget
        label->set_valign(
            Gtk::Align::START
        );

        label->set_wrap(
            true
        );

        label->set_selectable(
            true
        );

        label->set_use_markup(
            true
        );

    set_child(
        * label
    );
}

Glib::ustring Gemini::to_pango_markup(
    const Glib::ustring & GEMTEXT
) {
    Glib::ustring markup;

    std::istringstream stream(
        GEMTEXT
    );

    std::string line;

    while (std::getline(stream, line))
    {
        // Convert links
        auto match = Glib::Regex::split_simple(
            R"regex(^=>\s*([^\s]+)(\s(\d{4}-\d{2}-\d{2}))?(\s(.+))?$)regex",
            line.c_str()
        );

        Glib::ustring address = "";
        Glib::ustring date    = "";
        Glib::ustring alt     = "";

        int index = 0;

        for (const Glib::ustring & VALUE : match)
        {
            switch (index)
            {
                case 1: address = VALUE; break;
                case 3: date    = VALUE; break;
                case 5: alt     = VALUE; break;
            }

            index++;
        }

        // Keep original on address not found in line
        if (address.empty())
        {
            markup.append(
                line
            );
        }

        // Make pango link
        else
        {
            // Crate link name
            Glib::ustring name;

            if (!date.empty())
            {
                name.append(
                    date
                );
            }

            if (!alt.empty())
            {
                name.append(
                    name.empty() ? alt
                                 : name + " " + alt // append (to date)
                );
            }

            // Create pango markup
            markup.append(
                Glib::ustring::sprintf(
                    "<a href=\"%s\" title=\"%s\">%s</a>",
                    Glib::Markup::escape_text(
                        address // @TODO to absolute
                    ),
                    Glib::Markup::escape_text(
                        address
                    ),
                    Glib::Markup::escape_text(
                        name
                    )
                )
            );
        }

        markup.append(
            "\n" // @TODO
        );
    }

    // Return original gemtext on failure or pango markup on success
    return markup.empty() ? GEMTEXT : markup;
}