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
void History::back()
{
    if (has_memory_back())
    {
        index--;
    }
}

void History::forward()
{
    if (has_memory_forward())
    {
        index++;
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
    historyBack->set_sensitive(
        has_memory_back()
    );

    historyForward->set_sensitive(
        has_memory_forward()
    );
}

// Getters
bool History::has_memory_back() // @TODO & MEMORY
{
    try
    {
        const History::Memory & MEMORY = get_memory_back();

        return true;
    }

    catch (const std::out_of_range & EXCEPTION)
    {
        return false;
    }
}

bool History::has_memory_forward() // @TODO & MEMORY
{
    try
    {
        const History::Memory & MEMORY = get_memory_forward();

        return true;
    }

    catch (const std::out_of_range & EXCEPTION)
    {
        return false;
    }
}

// Copying getters
Glib::ustring History::make_memory_back_request()
{
    Glib::ustring request;

    if (has_memory_back())
    {
        request = get_memory_back().request;
    }

    return request;
}

Glib::ustring History::make_memory_forward_request()
{
    Glib::ustring request;

    if (has_memory_forward())
    {
        request = get_memory_forward().request;
    }

    return request;
}

// Private helpers
History::Memory & History::get_memory_back()
{
    return memory.at(
        index - 1
    );
}

History::Memory & History::get_memory_forward()
{
    return memory.at(
        index + 1
    );
}