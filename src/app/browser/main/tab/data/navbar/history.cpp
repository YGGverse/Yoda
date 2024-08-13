#include "history.hpp"
#include "history/back.hpp"
#include "history/forward.hpp"

using namespace app::browser::main::tab::data::navbar;

History::History()
{
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
