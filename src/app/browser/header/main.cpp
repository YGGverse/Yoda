#include "main.hpp"
#include "main/title.hpp"
#include "main/subtitle.hpp"

using namespace app::browser::header;

Main::Main()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    set_homogeneous(
        HOMOGENEOUS
    );

    // Init title
    mainTitle = new main::Title();

    append(
        * mainTitle
    );

    mainSubtitle = new main::Subtitle();

    append(
        * mainSubtitle
    );
}

Main::~Main()
{
    delete mainTitle;
    delete mainSubtitle;
}

void Main::set_title(
    const Glib::ustring & TEXT
) {
    mainTitle->set(
        TEXT
    );
}