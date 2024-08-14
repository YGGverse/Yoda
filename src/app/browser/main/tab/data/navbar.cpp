#include "navbar.hpp"
#include "navbar/base.hpp"
#include "navbar/bookmark.hpp"
#include "navbar/history.hpp"
#include "navbar/request.hpp"
#include "navbar/update.hpp"

#include "../../../../../lib/url.hpp"

using namespace app::browser::main::tab::data;

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
            sigc::mem_fun(
                * this,
                & Navbar::refresh
            )
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
    // Detect request has value
    bool has_request = request->get_text_length();

    // Toggle base button sensibility
    auto url = new ::lib::Url(
        request->get_text()
    );

    base->set_sensitive(
        has_request && !url->host.empty()
                    && !url->path.empty()
    );

    delete url;

    // Toggle update button sensibility
    update->set_sensitive(
        has_request
    );
}

// Getters
Glib::ustring Navbar::get_request_value()
{
    return request->get_text();
}