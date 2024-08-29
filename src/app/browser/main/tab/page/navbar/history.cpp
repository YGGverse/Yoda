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
void History::push(
    const Glib::ustring & VALUE
) {
    // @TODO
}

void History::refresh()
{
    historyBack->set_sensitive(
        false // @TODO
    );

    historyForward->set_sensitive(
        false // @TODO
    );
}