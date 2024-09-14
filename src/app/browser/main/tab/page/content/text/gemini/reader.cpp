#include "reader.hpp"

using namespace app::browser::main::tab::page::content::text::gemini;

Reader::Reader(
    const Glib::ustring & GEMTEXT,
    Glib::ustring & title,
    GUri * base
) {
    // Build markup
    Glib::ustring markup;

    std::istringstream stream(
        GEMTEXT
    );

    std::string line;

    while (std::getline(stream, line))
    {
        // Header
        int level;
        Glib::ustring header;

        if (Line::Match::header(line, level, header))
        {
            markup.append(
                Make::header(
                    level,
                    header
                )
            );

            if (title.empty())
            {
                title = header;
            }

            continue;
        }

        // Link
        Glib::ustring address;
        Glib::ustring date;
        Glib::ustring alt;

        if (Line::Match::link(line, address, date, alt))
        {
            markup.append(
                Make::link(
                    g_uri_to_string(
                        base
                    ),
                    address,
                    date,
                    alt
                )
            );

            continue;
        }

        // Quote
        Glib::ustring quote;

        if (Line::Match::quote(line, quote))
        {
            markup.append(
                Make::quote(
                    quote
                )
            );

            continue;
        }

        // @TODO other tags..

        // Default
        markup.append(
            Make::plain(
                line
            )
        );
    }

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
        markup
    );

    // Connect CSS
    auto css = Gtk::CssProvider::create();

        css->load_from_path(
            "src/app/browser/main/tab/page/content/text/gemini/reader.css" // @TODO
        );

        get_style_context()->add_provider(
            css,
            GTK_STYLE_PROVIDER_PRIORITY_APPLICATION
        );

    // Connect signals
    signal_activate_link().connect(
        [this](const Glib::ustring & URI) -> bool
        {
            // Open link URI
            activate_action(
                "win.open",
                Glib::Variant<Glib::ustring>::create(
                    URI
                )
            );

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

bool Reader::Line::Match::quote(
    const Glib::ustring & GEMTEXT,
    Glib::ustring & quote
) {
    auto match = Glib::Regex::split_simple(
        R"regex(^>(.*)$)regex",
        GEMTEXT
    );

    int index = 0; for (const Glib::ustring & MATCH : match)
    {
        switch (index)
        {
            case 1: quote = MATCH; break;
        }

        index++;
    }

    return !quote.empty();
}

// Markup tools
Glib::ustring Reader::Make::header(
    const int & LEVEL,
    const Glib::ustring & TEXT
) {
    switch (LEVEL)
    {
        case 1:

            return Glib::ustring::sprintf(
                "<span size=\"xx-large\">%s</span>\n",
                Glib::Markup::escape_text(
                    TEXT
                )
            );

        case 2:

            return Glib::ustring::sprintf(
                "<span size=\"x-large\">%s</span>\n",
                Glib::Markup::escape_text(
                    TEXT
                )
            );

        case 3:

            return Glib::ustring::sprintf(
                "<span size=\"large\">%s</span>\n",
                Glib::Markup::escape_text(
                    TEXT
                )
            );

        default:

            throw _("Header level not supported"); // @TODO
    }
}

Glib::ustring Reader::Make::link(
    const Glib::ustring & BASE,
    const Glib::ustring & ADDRESS,
    const Glib::ustring & DATE,
    const Glib::ustring & ALT
) {
    // Compose link description using optional date/alt values
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
            description.empty() ? ALT : description + " " + ALT
        );
    }

    // Make relative links absolute using base given
    const auto ABSOLUTE = g_uri_resolve_relative(
        BASE.c_str(),
        ADDRESS.c_str(),
        G_URI_FLAGS_NONE,
        NULL // GError * @TODO
    );

    return Glib::ustring::sprintf(
        "<a href=\"%s\" title=\"%s\">%s</a>\n",
        Glib::Markup::escape_text(
            ABSOLUTE == NULL ? ADDRESS : ABSOLUTE // @TODO exception?
        ),
        Glib::Markup::escape_text(
            ADDRESS
        ),
        Glib::Markup::escape_text(
            description
        )
    );
}

Glib::ustring Reader::Make::plain(
    const Glib::ustring & TEXT
) {
    return Glib::ustring::sprintf(
        "%s\n",
        Glib::Markup::escape_text(
            TEXT
        )
    );
}

Glib::ustring Reader::Make::quote(
    const Glib::ustring & TEXT
) {
    return Glib::ustring::sprintf(
        "<i>%s</i>\n",
        Glib::Markup::escape_text(
            TEXT
        )
    );
}