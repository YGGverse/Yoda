#include "base.hpp"

using namespace app::browser::main::tab::page::navigation;

Base::Base()
{
    set_icon_name(
        "go-home-symbolic"
    );

    set_tooltip_text(
        _("Base")
    );

    set_sensitive(
        false
    );

    // @TODO add action
}

void Base::update(
    const Glib::ustring & URI
) {
    GUri * uri = g_uri_parse(
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