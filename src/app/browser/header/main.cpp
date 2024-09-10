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

    set_valign(
        Gtk::Align::CENTER
    );

    set_homogeneous(
        HOMOGENEOUS
    );

    // Init title
    mainTitle = Gtk::make_managed<main::Title>();

    append(
        * mainTitle
    );

    mainSubtitle = Gtk::make_managed<main::Subtitle>();

    append(
        * mainSubtitle
    );
}

void Main::set_title(
    const Glib::ustring & VALUE
) {
    mainTitle->set(
        VALUE
    );
}

void Main::set_subtitle(
    const Glib::ustring & VALUE
) {
    mainSubtitle->set(
        VALUE
    );
}