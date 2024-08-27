#include "tab.hpp"
#include "tab/label.hpp"
#include "tab/page.hpp"

using namespace app::browser::main;

Tab::Tab()
{
    // Init widget
    set_scrollable(
        SCROLLABLE
    );

    // Init events
    signal_switch_page().connect(
        [this](Gtk::Widget * pageWidget, guint page_number)
        {
            refresh(
                page_number
            );

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
    const int & page_number
) {
    auto pageWidget = get_nth_page(
        page_number
    );

    if (pageWidget != nullptr)
    {
        return get_tab_label_text(
            * pageWidget
        );
    }

    return ""; // @TODO
};

// Actions
void Tab::append(
    const Glib::ustring & page_navbar_request_text,
    bool focus
) {
    auto tabPage  = new tab::Page(
        page_navbar_request_text
    );

    int page_number = append_page(
        * tabPage,
        * new tab::Label
    );

    set_tab_reorderable(
        * tabPage,
        REORDERABLE
    );

    if (focus)
    {
        set_current_page(
            page_number
        );
    }
};

void Tab::close(
    const int & page_number
) {
    remove_page(
        page_number
    );

    // @TODO memory cleanup
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
    const int & page_number
) {
    auto pageWidget = get_nth_page(
        page_number
    );

    if (pageWidget != nullptr)
    {
        pageWidget->activate_action(
            "page.update"
        );
    }

} // @TODO

void Tab::refresh(
    const int & page_number
) {
    auto pageWidget = get_nth_page(
        page_number
    );

    if (pageWidget == nullptr)
    {
        throw _("Tab page not found!");
    }

    auto labelWidget = get_tab_label(
        * pageWidget
    );

    if (labelWidget == nullptr)
    {
        throw _("Tab label not found!");
    }

    auto tabPage = (tab::Page *) pageWidget;

    auto tabLabel = (tab::Label *) labelWidget;

    tabLabel->set_label(
        tabPage->get_title()
    );

    // @TODO delegate refresh action to child level widgets (on available)
}