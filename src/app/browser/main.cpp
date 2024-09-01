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
    mainTab = new main::Tab();

    append(
        * mainTab
    );
}

Main::~Main()
{
    delete mainTab;
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
void Main::tab_append()
{
    mainTab->append(
        _("New page")
    );
};

void Main::tab_update()
{
    mainTab->update(
        mainTab->get_current_page()
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

void Main::tab_history_back()
{
    mainTab->back(
        mainTab->get_current_page()
    );
};

void Main::tab_history_forward()
{
    mainTab->forward(
        mainTab->get_current_page()
    );
};

void Main::refresh()
{
    mainTab->refresh(
        mainTab->get_current_page()
    );
};