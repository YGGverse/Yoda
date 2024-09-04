#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::page::navigation;

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
void History::add(
    const Glib::ustring & REQUEST,
    const bool & FOLLOW
) {
    memory.push_back(
        {
            REQUEST,
            std::time(
                nullptr
            ),
            true
        }
    );

    if (FOLLOW)
    {
        index = memory.size() - 1;
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


bool History::try_back(
    Memory & match,
    const bool & FOLLOW
) {
    try
    {
        match = memory.at(
            index - 1
        );

        if (FOLLOW)
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
    const bool & FOLLOW
) {
    try
    {
        match = memory.at(
            index + 1
        );

        if (FOLLOW)
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