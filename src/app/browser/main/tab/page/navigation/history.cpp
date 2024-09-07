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
void History::refresh()
{
    Memory match;

    historyBack->set_sensitive( // @TODO operate with action status
        try_back(
            match,
            false
        )
    );

    historyForward->set_sensitive( // @TODO operate with action status
        try_forward(
            match,
            false
        )
    );
}

void History::add(
    const Glib::ustring & REQUEST,
    const bool & UPDATE_MEMORY_INDEX
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

    if (UPDATE_MEMORY_INDEX)
    {
        index = memory.size() - 1;
    }
}

bool History::try_back(
    Memory & match,
    const bool & UPDATE_MEMORY_INDEX
) {
    try
    {
        match = memory.at(
            index - 1
        );

        if (UPDATE_MEMORY_INDEX)
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
    const bool & UPDATE_MEMORY_INDEX
) {
    try
    {
        match = memory.at(
            index + 1
        );

        if (UPDATE_MEMORY_INDEX)
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