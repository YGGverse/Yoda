#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::page::navbar;

History::History()
{
    add_css_class(
        "linked" // merge children elements
    );

    back = new history::Back();

        append(
            * back
        );

    forward = new history::Forward();

        append(
            * forward
        );
}

History::~History()
{
    delete back;
    delete forward;
};

// Actions
void History::push(
    const Glib::ustring & VALUE
) {
    // @TODO
}

void History::refresh()
{
    back->set_sensitive(
        false // @TODO
    );

    forward->set_sensitive(
        false // @TODO
    );
}