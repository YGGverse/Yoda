#include "tab.hpp"
#include "tab/data.hpp"
#include "tab/label.hpp"

using namespace app::browser::main;

Tab::Tab()
{
    set_scrollable(
        true
    );
}

Tab::~Tab() = default;

void Tab::append(
    const char * request,
    bool open,
    bool focus
) {
    label = new tab::Label();

    data = new tab::Data();

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