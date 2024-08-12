#include "tab.hpp"

using namespace app::browser::header;

Tab::Tab()
{
    set_tooltip_text(
        TOOLTIP
    );

    set_icon_name(
        ICON
    );

    signal_clicked().connect(
        sigc::mem_fun(
            * this,
            & Tab::click
        )
    );
}

Tab::~Tab() = default;

void Tab::click()
{
    // @TODO
}