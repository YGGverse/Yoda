#include "base.hpp"

using namespace app::browser::main::tab::page::navigation;

Base::Base(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__OPEN_LINK_VARIANT
) {
    // Init shared actions
    action__open_link_variant = ACTION__OPEN_LINK_VARIANT;

    // Init widget
    set_icon_name(
        "go-home-symbolic"
    );

    set_tooltip_text(
        _("Base")
    );

    set_sensitive(
        false
    );

    // Init events
    signal_clicked().connect(
        [this]
        {
            // Currently, there is
            action__open_link_variant->activate_variant(
                Glib::Variant<Glib::ustring>::create(
                    Glib::ustring::sprintf(
                        "%s://%s/",
                        g_uri_get_scheme(uri), // @TODO NULL validate?
                        g_uri_get_host(uri)
                    ) // at this moment, there is no G_URI_HIDE_*HOST option for g_uri_to_string_partial,
                      // build address manually using sprintf @TODO
                )
            );
        }
    );
}

void Base::update(
    const Glib::ustring & URI
) {
    uri = g_uri_parse(
        URI.c_str(),
        G_URI_FLAGS_NONE,
        NULL // @TODO GError *
    );

    set_sensitive(
        NULL != uri &&
        NULL != g_uri_get_host(uri) &&
        NULL != g_uri_get_path(uri)
    );
}