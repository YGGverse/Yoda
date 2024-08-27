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
        true
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
Glib::ustring Main::get_current_tab_label_text()
{
    int page_number = mainTab->get_current_page();

    return mainTab->get_label_text(
        page_number
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