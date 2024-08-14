#include "tab.hpp"
#include "tab/data.hpp"
#include "tab/label.hpp"

using namespace app::browser::main;

Tab::Tab()
{
    // Init widget
    set_scrollable(
        true
    );

    // Init events
    signal_switch_page().connect(
        sigc::mem_fun(
            * this,
            & Tab::on_switch
        )
    );
}

Tab::~Tab() = default;

// Actions
void Tab::append(
    const char * request,
    bool open,
    bool focus
) {
    auto label = new tab::Label();
    auto data  = new tab::Data();

    append_page(
        * data,
        * label
    );

    set_tab_reorderable(
        * data,
        true
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

void Tab::close(
    int number
) {
    remove_page(
        number
    );

    // @TODO clean memory
    // @TODO fix GtkGizmo reported min height, but sizes must be >= 0
}

void Tab::close_left()
{} // @TODO

void Tab::close_right()
{} // @TODO

void Tab::close_all()
{
    while (0 <= get_current_page())
    {
        close(
            -1 // last
        );
    }
}

void Tab::update(
    int number
) {
    auto page = get_nth_page(
        number
    );

    page->activate_action(
        "tab.update"
    );

} // @TODO

// Events
void Tab::on_switch(
    Gtk::Widget * page,
    guint page_num
) {
    // @TODO update header
}