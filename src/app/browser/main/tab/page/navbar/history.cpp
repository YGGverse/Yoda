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

History::~History() = default;