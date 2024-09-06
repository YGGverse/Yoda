#include "request.hpp"

using namespace app::browser::main::tab::page::navigation;

// Construct
Request::Request(
    const Glib::ustring & TEXT
) {
    // Init entry
    set_placeholder_text(
        _("URL or search term...")
    );

    set_hexpand(
        HEXPAND
    );

    set_progress_pulse_step(
        PROGRESS_PULSE_STEP
    );

    if (!TEXT.empty())
    {
        set_text(
            TEXT
        );

        parse();
    }

    // Connect events
    signal_changed().connect(
        [this]
        {
            parse();

            activate_action(
                "navigation.refresh"
            );
        }
    );

    signal_activate().connect(
        [this]
        {
            parse();

            activate_action(
                "win.main_tab_page_navigation_update"
            );
        }
    );
}

// Actions
void Request::refresh(
    const double & PROGRESS_FRACTION
) {
    // Update progress
    progress_fraction = PROGRESS_FRACTION;

    // Animate progress function
    Glib::signal_timeout().connect(
        [this]() -> bool
        {
            double current_progress_fraction = get_progress_fraction();

            // Animation in progress
            if (current_progress_fraction < progress_fraction)
            {
                set_progress_fraction(
                    current_progress_fraction + PROGRESS_PULSE_STEP
                );

                return true; // continue
            }

            // 100% of value, reset
            set_progress_fraction(
                progress_fraction = 0
            );

            return false; // stop
        },
        PROGRESS_ANIMATION_TIME
    );
}

// Getters
Glib::ustring Request::get_scheme()
{
    return scheme;
}

Glib::ustring Request::get_host()
{
    return host;
}

Glib::ustring Request::get_port()
{
    return port;
}

Glib::ustring Request::get_path()
{
    return path;
}

Glib::ustring Request::get_query()
{
    return path;
}

// Private helpers
void Request::parse()
{
    auto match = Glib::Regex::split_simple(
        R"regex(^((\w+)?:\/\/)?([^:\/]+)?(:(\d+)?)?([^\?$]+)?(\?(.*)?)?)regex",
        get_text()
    );

    scheme = "";
    host   = "";
    port   = "";
    path   = "";
    query  = "";

    int index = 0;

    for (const Glib::ustring & VALUE : match)
    {
        switch (index)
        {
            case 2: scheme = VALUE; break;
            case 3: host   = VALUE; break;
            case 5: port   = VALUE; break;
            case 6: path   = VALUE; break;
            case 8: query  = VALUE; break;
        }

        index++;
    }
}