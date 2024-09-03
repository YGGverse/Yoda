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
bool History::back(
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

bool History::forward(
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
    const Glib::ustring & REQUEST
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

        index = memory.size(); // @TODO
    }
}

void History::refresh()
{
    Memory match;

    historyBack->set_sensitive(
        back(
            match,
            false
        )
    );

    historyForward->set_sensitive(
        forward(
            match,
            false
        )
    );
}