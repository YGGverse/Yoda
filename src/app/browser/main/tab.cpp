#include "tab.hpp"
#include "tab/label.hpp"
#include "tab/page.hpp"

using namespace app::browser::main;

Tab::Tab()
{
    // Init widget
    set_scrollable(
        true
    );

    // Init events
    signal_switch_page().connect(
        [this](Gtk::Widget * page, guint page_num)
        {
            on_switch(
                page,
                page_num
            );
        }
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
    auto page  = new tab::Page();

    append_page(
        * page,
        * label
    );

    set_tab_reorderable(
        * page,
        true
    );

    if (focus)
    {
        set_current_page(
            page_num(
                * page
            )
        );
    }
};

void Tab::close(
    int number
) {
    auto page = get_nth_page(
        number
    );

    auto label = get_tab_label(
        * page
    );

    delete page;
    delete label;

    remove_page(
        number
    );

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
        "page.update"
    );

} // @TODO

// Events
void Tab::on_switch(
    Gtk::Widget * page,
    guint page_num
) {
    // @TODO update header text
}