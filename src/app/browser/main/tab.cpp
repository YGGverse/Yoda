#include "tab.hpp"
#include "gtkmm/label.h"

using namespace app::browser::main;

Tab::Tab()
{
    set_scrollable(
        SCROLLABLE
    );
}

Tab::~Tab() = default;

void Tab::append(
    const char * request,
    bool open,
    bool focus
) {
    Gtk::Label * data = new Gtk::Label("data"); // @TODO

    append_page(
        * data,
        * new Gtk::Label(
            LABEL
        )
    );

    set_tab_reorderable(
        * data,
        REORDERABLE
    );

    if (focus)
    {
        set_current_page(
            page_num(
                * data
            )
        );
    }
};