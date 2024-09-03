#include "navbar.hpp"
#include "navbar/base.hpp"
#include "navbar/bookmark.hpp"
#include "navbar/history.hpp"
#include "navbar/request.hpp"
#include "navbar/update.hpp"

using namespace app::browser::main::tab::page;

Navbar::Navbar(
    const Glib::ustring & REQUEST
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
    navbarBase = Gtk::make_managed<navbar::Base>();

        append(
            * navbarBase
        );

    navbarHistory = Gtk::make_managed<navbar::History>();

        append(
            * navbarHistory
        );

    navbarUpdate = Gtk::make_managed<navbar::Update>();

        append(
            * navbarUpdate
        );

    navbarRequest = Gtk::make_managed<navbar::Request>(
        REQUEST
    );

        append(
            * navbarRequest
        );

    navbarBookmark = Gtk::make_managed<navbar::Bookmark>();

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

// Actions
void Navbar::back()
{
    if (navbarHistory->has_memory_back())
    {
        navbarRequest->set_text(
            navbarHistory->make_memory_back_request()
        );

        navbarHistory->back(); // --

        navbarUpdate->activate();
    }
}

void Navbar::forward()
{
    if (navbarHistory->has_memory_forward())
    {
        navbarRequest->set_text(
            navbarHistory->make_memory_forward_request()
        );

        navbarHistory->forward(); // ++

        navbarUpdate->activate();
    }
}

void Navbar::push(
    const Glib::ustring & VALUE
) {
    navbarHistory->push(
        VALUE
    );
}

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

    // Refresh history widget
    navbarHistory->refresh();
}

// Setters @TODO is really wanted?
void Navbar::set_request_text(
    const Glib::ustring & VALUE
) {
    navbarRequest->set_text(
        VALUE
    );

    // refresh(); not wanted as on change listener do same @TODO
}

// Getters @TODO &
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