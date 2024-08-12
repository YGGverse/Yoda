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
    remove(
        * tab
    );

    delete tab;

    tab = nullptr;
}

void Main::tab_append()
{
    tab->append(
        nullptr,
        true,
        true
    );
};

void Main::tab_close()
{
    tab->close();
};