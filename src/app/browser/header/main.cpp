#include "main.hpp"
#include "main/title.hpp"

using namespace app::browser::header;

Main::Main()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init title
    title = new main::Title();

    append(
        * title
    );
}

Main::~Main()
{
    delete title;
}

void Main::set_title(
    const Glib::ustring text
) {
    title->set_text(
        text
    );
}