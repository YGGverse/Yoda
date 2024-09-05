#include "navigation.hpp"
#include "navigation/base.hpp"
#include "navigation/bookmark.hpp"
#include "navigation/history.hpp"
#include "navigation/request.hpp"
#include "navigation/update.hpp"

using namespace app::browser::main::tab::page;

Navigation::Navigation(
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
    navigationBase = Gtk::make_managed<navigation::Base>();

        append(
            * navigationBase
        );

    navigationHistory = Gtk::make_managed<navigation::History>();

        append(
            * navigationHistory
        );

    navigationUpdate = Gtk::make_managed<navigation::Update>();

        append(
            * navigationUpdate
        );

    navigationRequest = Gtk::make_managed<navigation::Request>(
        REQUEST
    );

        append(
            * navigationRequest
        );

    navigationBookmark = Gtk::make_managed<navigation::Bookmark>();

        append(
            * navigationBookmark
        );

    // Init actions group
    auto GioSimpleActionGroup = Gio::SimpleActionGroup::create();

        // Define group actions
        GioSimpleActionGroup->add_action(
            "refresh",
            [this]
            {
                refresh();
            }
        );

    insert_action_group(
        "navigation",
        GioSimpleActionGroup
    );
}

// Actions
bool Navigation::history_try_back()
{
    navigation::History::Memory match;

    if (navigationHistory->try_back(match))
    {
        navigationRequest->set_text(
            match.request
        );

        return activate_action(
            "win.main_tab_page_navigation_update"
        );
    }

    return false;
}

bool Navigation::history_try_forward()
{
    navigation::History::Memory match;

    if (navigationHistory->try_forward(match))
    {
        navigationRequest->set_text(
            match.request
        );

        return activate_action(
            "win.main_tab_page_navigation_update"
        );
    }

    return false;
}

void Navigation::history_add(
    const Glib::ustring & VALUE
) {
    navigationHistory->add(
        VALUE
    );
}

void Navigation::refresh()
{
    // Toggle base button sensibility
    navigationBase->set_sensitive(
        !navigationRequest->get_host().empty() && !navigationRequest->get_path().empty()
    );

    // Toggle update button sensibility
    navigationUpdate->set_sensitive(
        navigationRequest->get_text_length() > 0
    );

    // Refresh history widget
    navigationHistory->refresh();
}

// Setters @TODO is really wanted?
void Navigation::set_request_text(
    const Glib::ustring & VALUE
) {
    navigationRequest->set_text(
        VALUE
    );

    // refresh(); not wanted as on change listener do same @TODO
}

// Getters @TODO &
Glib::ustring Navigation::get_request_text()
{
    return navigationRequest->get_text();
}

Glib::ustring Navigation::get_request_scheme()
{
    return navigationRequest->get_scheme();
}

Glib::ustring Navigation::get_request_host()
{
    return navigationRequest->get_host();
}

Glib::ustring Navigation::get_request_path()
{
    return navigationRequest->get_path();
}

Glib::ustring Navigation::get_request_query()
{
    return navigationRequest->get_query();
}

Glib::ustring Navigation::get_request_port()
{
    return navigationRequest->get_port();
}