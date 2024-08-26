#include "navbar.hpp"
#include "navbar/base.hpp"
#include "navbar/bookmark.hpp"
#include "navbar/history.hpp"
#include "navbar/request.hpp"
#include "navbar/update.hpp"

using namespace app::browser::main::tab::page;

Navbar::Navbar(
    const Glib::ustring & request_text
) {
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
    navbarBase = new navbar::Base();

        append(
            * navbarBase
        );

    navbarHistory = new navbar::History();

        append(
            * navbarHistory
        );

    navbarUpdate = new navbar::Update();

        append(
            * navbarUpdate
        );

    navbarRequest = new navbar::Request(
        request_text
    );

        append(
            * navbarRequest
        );

    navbarBookmark = new navbar::Bookmark();

        append(
            * navbarBookmark
        );

    // Init actions group
    auto GioSimpleActionGroup_RefPtr = Gio::SimpleActionGroup::create();

        // Define group actions
        GioSimpleActionGroup_RefPtr->add_action(
            "refresh",
            [this]
            {
                refresh();
            }
        );

    insert_action_group(
        "navbar",
        GioSimpleActionGroup_RefPtr
    );
}

Navbar::~Navbar()
{
    delete navbarBase;
    delete navbarBookmark;
    delete navbarHistory;
    delete navbarRequest;
    delete navbarUpdate;
};

// Actions
void Navbar::refresh()
{
    // Toggle base button sensibility
    navbarBase->set_sensitive(
        !navbarRequest->get_host().empty() && !navbarRequest->get_path().empty()
    );

    // Toggle update button sensibility
    navbarUpdate->set_sensitive(
        navbarRequest->get_text_length() > 0
    );
}

// Setters
void Navbar::set_request_text(
    const Glib::ustring & value
) {
    navbarRequest->set_text(
        value
    );

    // refresh(); not wanted as on change listener do same @TODO
}

// Getters
Glib::ustring Navbar::get_request_text()
{
    return navbarRequest->get_text();
}

Glib::ustring Navbar::get_request_scheme()
{
    return navbarRequest->get_scheme();
}

Glib::ustring Navbar::get_request_host()
{
    return navbarRequest->get_host();
}

Glib::ustring Navbar::get_request_path()
{
    return navbarRequest->get_path();
}

Glib::ustring Navbar::get_request_query()
{
    return navbarRequest->get_query();
}

Glib::ustring Navbar::get_request_port()
{
    return navbarRequest->get_port();
}