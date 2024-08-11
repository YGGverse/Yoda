#include "main.hpp"
#include "main/tab.hpp"

using namespace app::browser;

Main::Main()
{
    // Init container
    set_homogeneous(
        true
    );

    // Init tabs
    tab = new main::Tab();

    append(
        * tab
    );
}

void Main::tabAppend()
{
    tab->append(
        nullptr,
        true,
        true
    );
};