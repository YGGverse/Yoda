#include "markup.hpp"

using namespace app::browser::main::tab::page::content::text::gemini;

Markup::Markup(
    const Glib::ustring & GEMTEXT
) {
    // Init widget
    set_valign(
        Gtk::Align::START
    );

    set_wrap(
        true
    );

    set_selectable(
        true
    );

    set_use_markup(
        true
    );

    set_markup(
        Markup::make(
            GEMTEXT
        )
    );

    // Connect signals
    signal_activate_link().connect(
        [this](const Glib::ustring & URI) -> bool
        {
            // @TODO follow action

            return false;
        },
        false // after
    );
}

// Match tools
bool Markup::Line::Match::link(
    const Glib::ustring & GEMTEXT,
    Glib::ustring & address,
    Glib::ustring & date,
    Glib::ustring & alt
) {
    auto match = Glib::Regex::split_simple(
        R"regex(^=>\s*([^\s]+)(\s(\d{4}-\d{2}-\d{2}))?(\s(.+))?$)regex",
        GEMTEXT
    );

    int index = 0; for (const Glib::ustring & MATCH : match)
    {
        switch (index)
        {
            case 1: address = MATCH; break;
            case 3: date    = MATCH; break;
            case 5: alt     = MATCH; break;
        }

        index++;
    }

    return !address.empty();
}

// Markup tools
Glib::ustring Markup::make(
    const Glib::ustring & GEMTEXT
) {
    Glib::ustring pango;

    std::istringstream stream(
        GEMTEXT
    );

    std::string line;

    while (std::getline(stream, line))
    {
        // Links
        Glib::ustring address;
        Glib::ustring date;
        Glib::ustring alt;

        if (Line::Match::link(line, address, date, alt))
        {
            pango.append(
                Markup::Make::link(
                    address,
                    date,
                    alt
                )
            );
        }

        else
        {
            pango.append(
                line
            );
        }

        // @TODO other tags..

        pango.append(
            "\n" // @TODO
        );
    }

    return pango;
}

Glib::ustring Markup::Make::link(
    const Glib::ustring & ADDRESS,
    const Glib::ustring & DATE,
    const Glib::ustring & ALT
) {
    Glib::ustring description;

    if (!DATE.empty())
    {
        description.append(
            DATE
        );
    }

    if (!ALT.empty())
    {
        description.append(
            description.empty() ? ALT : description + " " + ALT // append (to date)
        );
    }

    return Glib::ustring::sprintf(
        "<a href=\"%s\" title=\"%s\">%s</a>",
        Glib::Markup::escape_text(
            ADDRESS // @TODO to absolute
        ),
        Glib::Markup::escape_text(
            ADDRESS
        ),
        Glib::Markup::escape_text(
            description
        )
    );
}