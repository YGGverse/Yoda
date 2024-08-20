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
            // Refresh window elements, e.g. tab label to header bar
            activate_action(
                "win.refresh"
            );
        }
    );
}

Tab::~Tab() = default;

// Getters
Glib::ustring Tab::get_label_text(
    int page_number
) {
    auto page = get_nth_page(
        page_number
    );

    if (page != nullptr)
    {
        return get_tab_label_text(
            * get_nth_page(
                page_number
            )
        );
    }

    return ""; // @TODO
};

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
    int page_number
) {
    auto page = get_nth_page(
        page_number
    );

    if (page != nullptr)
    {
        page->activate_action(
            "page.update"
        );
    }

} // @TODO