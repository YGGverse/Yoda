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
        Glib::ustring uri;
        Glib::ustring date;
        Glib::ustring alt;

        if (Line::Match::link(line, uri, date, alt))
        {
            markup.append(
                Make::link(
                    base,
                    uri,
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
    Glib::ustring & uri,
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
            case 1: uri = MATCH; break;
            case 3: date    = MATCH; break;
            case 5: alt     = MATCH; break;
        }

        index++;
    }

    return !uri.empty();
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
    GUri * base,
    const Glib::ustring & URI,
    const Glib::ustring & DATE,
    const Glib::ustring & ALT
) {
    // Convert relative URI to absolute
    const gchar * ABSOLUTE_URI = g_uri_resolve_relative(
        g_uri_to_string( // seems could be NULL
            base
        ),
        URI.c_str(),
        G_URI_FLAGS_NONE,
        NULL // GError * @TODO
    );

        // Validate conversion success to continue
        if (ABSOLUTE_URI == NULL)
        {
            throw _("Could not resolve absolute URI"); // @TODO
        }

    // Get host from base
    const gchar * BASE_HOST = g_uri_get_host(
        base
    );

        // Validate base host parsed
        if (BASE_HOST == NULL)
        {
            throw _("Could not parse base host"); // @TODO
        }


    // Create GUri pointer from absolute URI
    GUri * absolute_uri_base = g_uri_parse(
        ABSOLUTE_URI,
        G_URI_FLAGS_NONE,
        NULL // GError * @TODO
    );

        // Validate it's created to continue
        if (ABSOLUTE_URI == NULL)
        {
            throw _("Could not create GUri pointer for absolute URI"); // @TODO
        }

    // Get host from absolute URI base
    const gchar * URI_HOST = g_uri_get_host(
        absolute_uri_base
    );

        // Validate base host parsed
        if (URI_HOST == NULL)
        {
            throw _("Could not parse absolute base host"); // @TODO
        }

    // Everything done, make link info
    Glib::ustring alt;

        // Indicate external links
        if (0 != strcmp(URI_HOST, BASE_HOST))
        {
            alt.append(
                "⇖"
            );
        }

        // Append date on available
        if (!DATE.empty())
        {
            alt.append(
                alt.empty() ? DATE : " " + DATE
            );
        }

        // Append alt text
        if (!ALT.empty())
        {
            alt.append(
                alt.empty() ? ALT : " " + ALT
            );
        }

    // Build markup and get result
    return Glib::ustring::sprintf(
        "<a href=\"%s\" title=\"%s\">%s</a>\n",
        Glib::Markup::escape_text(
            ABSOLUTE_URI
        ),
        Glib::Markup::escape_text(
            URI
        ),
        Glib::Markup::escape_text(
            alt
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