#include "reader.hpp"

using namespace app::browser::main::tab::page::content::text::gemini;

Reader::Reader(
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
        make(
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
bool Reader::Line::Match::header(
    const Glib::ustring & GEMTEXT,
    int & level,
    Glib::ustring & text
) {
    auto match = Glib::Regex::split_simple(
        R"regex(^(#{1,3})(.*)$)regex",
        GEMTEXT
    );

    int index = 0; for (const Glib::ustring & MATCH : match)
    {
        switch (index)
        {
            case 1: level = MATCH.length(); break;
            case 2: text  = MATCH; break;
        }

        index++;
    }

    return level > 0 && !text.empty();
}

bool Reader::Line::Match::link(
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
Glib::ustring Reader::make(
    const Glib::ustring & GEMTEXT
) {
    Glib::ustring pango;

    std::istringstream stream(
        GEMTEXT
    );

    std::string line;

    while (std::getline(stream, line))
    {
        // Header
        int level;
        Glib::ustring text;

        if (Line::Match::header(line, level, text))
        {
            pango.append(
                Make::header(
                    level,
                    text
                )
            );

            continue;
        }

        // Link
        Glib::ustring address;
        Glib::ustring date;
        Glib::ustring alt;

        if (Line::Match::link(line, address, date, alt))
        {
            pango.append(
                Make::link(
                    address,
                    date,
                    alt
                )
            );

            continue;
        }

        // @TODO other tags..

        pango.append(
            line.append(
                "\n"
            ) // @TODO
        );
    }

    return pango;
}

Glib::ustring Reader::Make::header(
    const int & LEVEL,
    const Glib::ustring & VALUE
) {
    switch (LEVEL)
    {
        case 1:

            return Glib::ustring::sprintf(
                "<span size=\"xx-large\">%s</span>\n",
                Glib::Markup::escape_text(
                    VALUE
                )
            );

        case 2:

            return Glib::ustring::sprintf(
                "<span size=\"x-large\">%s</span>\n",
                Glib::Markup::escape_text(
                    VALUE
                )
            );

        case 3:

            return Glib::ustring::sprintf(
                "<span size=\"large\">%s</span>\n",
                Glib::Markup::escape_text(
                    VALUE
                )
            );

        default:

            throw _("Header level not supported"); // @TODO
    }
}

Glib::ustring Reader::Make::link(
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
        "<a href=\"%s\" title=\"%s\">%s</a>\n",
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