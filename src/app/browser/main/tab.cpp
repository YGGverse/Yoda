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
            // Refresh window elements, e.g. tab label to header bar
            activate_action(
                "win.refresh"
            );
        }
    );
}

Tab::~Tab() = default;

// Getters
Glib::ustring Tab::get_page_title(
    const int & PAGE_NUMBER
) {
    return get_tabPage(PAGE_NUMBER)->get_title();
};

Glib::ustring Tab::get_page_subtitle(
    const int & PAGE_NUMBER
) {
    return get_tabPage(PAGE_NUMBER)->get_subtitle();
};

// Actions
void Tab::append(
    const Glib::ustring & TITLE,
    const Glib::ustring & REQUEST,
    const bool & FOCUS
) {
    auto tabPage = new tab::Page(
        TITLE,
        REQUEST
    );

    auto tabLabel = new tab::Label(
        TITLE
    );

    int page_number = append_page(
        * tabPage,
        * tabLabel
    );

    set_tab_reorderable(
        * tabPage,
        REORDERABLE
    );

    if (FOCUS)
    {
        set_current_page(
            page_number
        );
    }
};

void Tab::close(
    const int & PAGE_NUMBER
) {
    remove_page(
        PAGE_NUMBER
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

void Tab::history_back(
    const int & PAGE_NUMBER
) {
    auto tabPage = get_tabPage(
        PAGE_NUMBER
    );

    // @TODO tabPage->back()

    // Refresh children widgets
    tabPage->refresh();
}

void Tab::history_forward(
    const int & PAGE_NUMBER
) {
    auto tabPage = get_tabPage(
        PAGE_NUMBER
    );

    // @TODO tabPage->forward()

    // Refresh children widgets
    tabPage->refresh();
}

void Tab::refresh(
    const int & PAGE_NUMBER
) {
    auto tabPage = get_tabPage(
        PAGE_NUMBER
    );

    get_tabLabel(PAGE_NUMBER)->set_label(
        tabPage->get_title()
    );

    // Refresh children widgets
    tabPage->refresh();
}

void Tab::update(
    const int & PAGE_NUMBER
) {
    auto pageWidget = get_nth_page(
        PAGE_NUMBER
    );

    if (pageWidget == nullptr)
    {
        throw _("Tab page not found!");
    }

    pageWidget->activate_action(
        "page.update"
    );
}

// Private helpers
tab::Label * Tab::get_tabLabel(
    const int & PAGE_NUMBER
) {
    auto pageWidget = get_nth_page(
        PAGE_NUMBER
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

    return (tab::Label *) labelWidget;
}

tab::Page * Tab::get_tabPage(
    const int & PAGE_NUMBER
) {
    auto pageWidget = get_nth_page(
        PAGE_NUMBER
    );

    if (pageWidget == nullptr)
    {
        throw _("Tab page not found!");
    }

    return (tab::Page *) pageWidget;
}