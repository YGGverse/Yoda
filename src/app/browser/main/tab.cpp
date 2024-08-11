#include "tab.hpp"
#include "gtkmm/label.h"

using namespace app::browser::main;

Tab::Tab()
{
    set_scrollable(
        SCROLLABLE
    );
}

void Tab::append(
    const char * request,
    bool open,
    bool focus
) {
    append_page( // @TODO
        * new Gtk::Label("data"),
        * new Gtk::Label("tab")
    );
};