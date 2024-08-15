#include "page.hpp"
#include "page/navbar.hpp"
#include "page/content.hpp"

using namespace app::browser::main::tab;

Page::Page()
{
    // Init container
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    // Init actions group
    action_group = Gio::SimpleActionGroup::create();

        // Define group actions
        action_group->add_action(
            "update",
            sigc::mem_fun(
                * this,
                & Page::update
            )
        );

    insert_action_group(
        "page",
        action_group
    );

    // Init components
    navbar = new page::Navbar();

        append(
            * navbar
        );

        // Refresh children elements view (e.g. buttons sensitivity)
        // because of insert_action_group + append here @TODO
        navbar->refresh();

    content = new page::Content();

        append(
            * content
        );
}

Page::~Page() = default;

// Actions
void Page::update()
{
    // navbar->get_request_value() @TODO
}
