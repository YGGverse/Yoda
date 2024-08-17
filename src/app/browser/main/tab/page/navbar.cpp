#include "navbar.hpp"
#include "navbar/base.hpp"
#include "navbar/bookmark.hpp"
#include "navbar/history.hpp"
#include "navbar/request.hpp"
#include "navbar/update.hpp"

using namespace app::browser::main::tab::page;

Navbar::Navbar()
{
    // Init container
    set_orientation(
        Gtk::Orientation::HORIZONTAL
    );

    set_spacing(
        SPACING
    );

    set_margin_top(
        MARGIN
    );

    set_margin_start(
        MARGIN
    );

    set_margin_end(
        MARGIN
    );

    set_margin_bottom(
        MARGIN
    );

    // Init components
    base = new navbar::Base();

        append(
            * base
        );

    history = new navbar::History();

        append(
            * history
        );

    update = new navbar::Update();

        append(
            * update
        );

    request = new navbar::Request();

        append(
            * request
        );

    bookmark = new navbar::Bookmark();

        append(
            * bookmark
        );

    // Init actions group
    action_group = Gio::SimpleActionGroup::create();

        // Define group actions
        action_group->add_action(
            "refresh",
            [this]
            {
                refresh();
            }
        );

    insert_action_group(
        "navbar",
        action_group
    );
}

Navbar::~Navbar() = default;

// Actions
void Navbar::refresh()
{
    // Toggle base button sensibility
    base->set_sensitive(
        !request->get_host().empty() && !request->get_path().empty()
    );

    // Toggle update button sensibility
    update->set_sensitive(
        (bool) request->get_text_length()
    );
}

// Setters
void Navbar::set_request(
    const Glib::ustring & value
) {
    request->set_text(
        value
    );

    // refresh(); not wanted as on change listener do same @TODO
}

// Getters
Glib::ustring Navbar::get_request()
{
    return request->get_text();
}

Glib::ustring Navbar::get_request_scheme()
{
    return request->get_scheme();
}

Glib::ustring Navbar::get_request_host()
{
    return request->get_host();
}

Glib::ustring Navbar::get_request_path()
{
    return request->get_path();
}

Glib::ustring Navbar::get_request_query()
{
    return request->get_query();
}

Glib::ustring Navbar::get_request_port()
{
    return request->get_port();
}