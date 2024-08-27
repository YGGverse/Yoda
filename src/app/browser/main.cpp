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
    tab = new main::Tab();

    append(
        * tab
    );
}

Main::~Main()
{
    delete tab;
}

// Getters
Glib::ustring Main::get_current_tab_label_text()
{
    int page_number = tab->get_current_page();

    return tab->get_label_text(
        page_number
    );
};

// Actions
void Main::tab_append()
{
    tab->append(
        _("New page")
    );
};

void Main::tab_update()
{
    tab->update(
        tab->get_current_page()
    );
};

void Main::tab_close()
{
    tab->close(
        tab->get_current_page()
    );
};

void Main::tab_close_left()
{
    tab->close_left();
};

void Main::tab_close_right()
{
    tab->close_right();
};

void Main::tab_close_all()
{
    tab->close_all();
};