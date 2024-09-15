#include "gemini.hpp"
#include "gemini/reader.hpp"

using namespace app::browser::main::tab::page::content::text;

Gemini::Gemini(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__OPEN_LINK_VARIANT,
    const Glib::ustring & GEMTEXT,
    Glib::ustring & title,
    GUri * uri
) : Gtk::Viewport( // add scrolled window features to childs
    NULL,
    NULL
) {
    // Init widget
    set_scroll_to_focus(
        false
    );

    set_child(
        * Gtk::make_managed<gemini::Reader>(
            ACTION__OPEN_LINK_VARIANT,
            GEMTEXT,
            title,
            uri
        )
    );
}