#include "main.hpp"
#include "main/tab.hpp"

using namespace app::browser;

Main::Main(
    sqlite3 * db,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        HOMOGENEOUS
    );

    // Init components
    mainTab = Gtk::make_managed<main::Tab>(
        db,
        ACTION__UPDATE,
        ACTION__MAIN_TAB_CLOSE_ACTIVE,
        ACTION__MAIN_TAB_CLOSE_ALL,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_RELOAD
    );

        append(
            * mainTab
        );
}

// Actions
void Main::update()
{
    mainTab->update(
        mainTab->get_current_page()
    );
};

void Main::tab_append()
{
    mainTab->append(
        _("New tab"),
        true
    );
};

void Main::clean()
{
    mainTab->clean();
};

void Main::restore()
{
    mainTab->restore();
};

void Main::save()
{
    mainTab->save();
};

void Main::tab_close()
{
    mainTab->close(
        mainTab->get_current_page()
    );
};

void Main::tab_close_left()
{
    mainTab->close_left();
};

void Main::tab_close_right()
{
    mainTab->close_right();
};

void Main::tab_close_all()
{
    mainTab->close_all();
};

void Main::tab_page_navigation_reload() {
    mainTab->page_navigation_reload(
        mainTab->get_current_page(), // @TODO
        true
    );
};

void Main::tab_page_navigation_history_back()
{
    mainTab->page_navigation_history_back(
        mainTab->get_current_page() // @TODO
    );
};

void Main::tab_page_navigation_history_forward()
{
    mainTab->page_navigation_history_forward(
        mainTab->get_current_page() // @TODO
    );
};

// Getters
Glib::ustring Main::get_current_tab_page_title()
{
    return mainTab->get_page_title(
        mainTab->get_current_page()
    );
};

Glib::ustring Main::get_current_tab_page_description()
{
    return mainTab->get_page_description(
        mainTab->get_current_page()
    );
};