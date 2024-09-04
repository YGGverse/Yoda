#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::page::navbar;

History::History()
{
    add_css_class(
        "linked" // merge children elements
    );

    historyBack = Gtk::make_managed<history::Back>();

        append(
            * historyBack
        );

    historyForward = Gtk::make_managed<history::Forward>();

        append(
            * historyForward
        );
}

// Actions
bool History::try_back(
    Memory & match,
    bool follow
) {
    try
    {
        match = memory.at(
            index - 1
        );

        if (follow)
        {
            index--;
        }

        return true;
    }

    catch (std::out_of_range)
    {
        return false;
    }
}

bool History::try_forward(
    Memory & match,
    bool follow
) {
    try
    {
        match = memory.at(
            index + 1
        );

        if (follow)
        {
            index++;
        }

        return true;
    }

    catch (std::out_of_range)
    {
        return false;
    }
}

void History::push(
    const Glib::ustring & REQUEST,
    bool follow
) {
    if (memory.empty() || memory.back().request != REQUEST)
    {
        memory.push_back(
            {
                REQUEST,
                std::time(
                    nullptr
                ),
                true
            }
        );

        if (follow)
        {
            index = memory.size();
        }
    }
}

void History::refresh()
{
    Memory match;

    historyBack->set_sensitive(
        try_back(
            match,
            false
        )
    );

    historyForward->set_sensitive(
        try_forward(
            match,
            false
        )
    );
}