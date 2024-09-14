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
        Markup::make(
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

        // Connect signals
        label->signal_activate_link().connect(
            [label](const Glib::ustring & URI) -> bool
            {
                // @TODO follow action

                return false;
            },
            false // after
        );

    set_child(
        * label
    );
}

// Match tools
bool Gemini::Line::Match::link(
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
Glib::ustring Gemini::Markup::make(
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

Glib::ustring Gemini::Markup::Make::link(
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