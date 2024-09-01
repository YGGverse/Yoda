#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::page::navbar;

History::History()
{
    add_css_class(
        "linked" // merge children elements
    );

    historyBack = new history::Back();

        append(
            * historyBack
        );

    historyForward = new history::Forward();

        append(
            * historyForward
        );
}

History::~History()
{
    delete historyBack;
    delete historyForward;
};

// Actions
void History::back()
{
    historyBack->activate();
}

void History::forward()
{
    historyForward->activate();
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
    }
}

void History::refresh()
{
    historyBack->set_sensitive(
        false // @TODO memory.size() > 0
    );

    historyForward->set_sensitive(
        false // @TODO memory.size() > 0
    );
}