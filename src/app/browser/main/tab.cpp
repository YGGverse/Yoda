#include "tab.hpp"
#include "tab/label.hpp"
#include "tab/page.hpp"

using namespace app::browser::main;

Tab::Tab(
    SQLite::Database & db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_UPDATE
) {
    // Init database
    db.exec(
        R"SQL(
            CREATE TABLE IF NOT EXISTS `app_browser_tab`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024)
            )
        )SQL"
    );

    // Init actions
    action__refresh                             = ACTION__REFRESH;
    action__tab_close_active                    = ACTION__TAB_CLOSE_ACTIVE;
    action__tab_close_all                       = ACTION__MAIN_TAB_CLOSE_ALL;
    action__tab_page_navigation_history_back    = ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK;
    action__tab_page_navigation_history_forward = ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD;
    action__tab_page_navigation_update          = ACTION__TAB_PAGE_NAVIGATION_UPDATE;

    // Init widget
    set_scrollable(
        SCROLLABLE
    );

    // Init event listeners
    signal_switch_page().connect(
        [this](Gtk::Widget*, guint)
        {
            // Refresh window elements, e.g. tab label to header bar
            action__refresh->activate();
        }
    );
}

// Actions
void Tab::refresh(
    const int & PAGE_NUMBER
) {
    auto tabPage = get_tabPage(
        PAGE_NUMBER
    );

    get_tabLabel(
        PAGE_NUMBER
    )->set_label(
        tabPage->get_title()
    );

    tabPage->refresh(
        tabPage->get_title(),
        tabPage->get_subtitle(),
        0 // @TODO
    );

    action__tab_close_active->set_enabled(
        get_n_pages() > 0
    );

    action__tab_close_all->set_enabled(
        get_n_pages() > 0
    );
}

void Tab::append(
    const bool & FOCUS
) {
    auto tabPage = new tab::Page(
        action__refresh,
        action__tab_page_navigation_history_back,
        action__tab_page_navigation_history_forward,
        action__tab_page_navigation_update
    );

    auto tabLabel = new tab::Label(
        action__tab_close_active
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

    // @TODO cleanup memory ot use managed children widgets
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

void Tab::page_navigation_update(
    const int & PAGE_NUMBER,
    const bool & ADD_HISTORY
) {
    get_tabPage(
        PAGE_NUMBER
    )->navigation_update(
        ADD_HISTORY
    );
}

void Tab::page_navigation_history_back(
    const int & PAGE_NUMBER
) {
    get_tabPage(
        PAGE_NUMBER
    )->navigation_history_back();
}

void Tab::page_navigation_history_forward(
    const int & PAGE_NUMBER
) {
    get_tabPage(
        PAGE_NUMBER
    )->navigation_history_forward();
}

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

// Private helpers
tab::Label * Tab::get_tabLabel(
    const int & PAGE_NUMBER
) {
    auto pageWidget = get_nth_page(
        PAGE_NUMBER
    );

    if (pageWidget == nullptr)
    {
        throw _("Tab page not found!");  // @TODO
    }

    auto labelWidget = get_tab_label(
        * pageWidget
    );

    if (labelWidget == nullptr)
    {
        throw _("Tab label not found!"); // @TODO
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
        throw _("Tab page not found!"); // @TODO
    }

    return (tab::Page *) pageWidget;
}