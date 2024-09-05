#include "main.hpp"
#include "main/tab.hpp"

using namespace app::browser;

Main::Main()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        HOMOGENEOUS
    );

    // Init tabs
    mainTab = Gtk::make_managed<main::Tab>();

    append(
        * mainTab
    );
}

// Getters
Glib::ustring Main::get_current_tab_page_title()
{
    return mainTab->get_page_title(
        mainTab->get_current_page()
    );
};

Glib::ustring Main::get_current_tab_page_subtitle()
{
    return mainTab->get_page_subtitle(
        mainTab->get_current_page()
    );
};

// Actions
void Main::refresh()
{
    mainTab->refresh(
        mainTab->get_current_page()
    );
};

void Main::tab_append()
{
    mainTab->append(
        _("New page")
    );
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

void Main::tab_page_navigation_update(
    const bool & ADD_HISTORY
) {
    mainTab->page_navigation_update(
        mainTab->get_current_page(), // @TODO
        ADD_HISTORY
    );
};

bool Main::tab_page_navigation_history_try_back()
{
    const int & PAGE_NUMBER = mainTab->get_current_page();

    if (PAGE_NUMBER >= 0)
    {
        return mainTab->page_navigation_history_try_back(
            PAGE_NUMBER
        );
    }

    return false;
};

bool Main::tab_page_navigation_history_try_forward()
{
    const int & PAGE_NUMBER = mainTab->get_current_page();

    if (PAGE_NUMBER >= 0)
    {
        return mainTab->page_navigation_history_try_forward(
            PAGE_NUMBER
        );

        return true;
    }

    return false;
};