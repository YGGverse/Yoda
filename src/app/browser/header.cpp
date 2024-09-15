#include "header.hpp"
#include "header/main.hpp"
#include "header/menu.hpp"
#include "header/tab.hpp"

using namespace app::browser;

Header::Header(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_CLEAN,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_RESTORE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_SAVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_APPEND,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init widget
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );

    // Init components
    headerMenu = Gtk::make_managed<header::Menu>(
        ACTION__DEBUG,
        ACTION__QUIT,
        ACTION__SESSION_CLEAN,
        ACTION__SESSION_RESTORE,
        ACTION__SESSION_SAVE,
        ACTION__TAB_APPEND,
        ACTION__TAB_CLOSE,
        ACTION__TAB_CLOSE_ALL,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__TAB_PAGE_NAVIGATION_RELOAD
    );

        pack_start(
            * headerMenu
        );

    headerTab = Gtk::make_managed<header::Tab>(
        ACTION__TAB_APPEND
    );

        pack_start(
            * headerTab
        );

    headerMain = Gtk::make_managed<header::Main>();

        set_title_widget(
            * headerMain
        );
}

void Header::update(
    const Glib::ustring & TITLE,
    const Glib::ustring & SUBTITLE
) {
    headerMain->set_title(
        TITLE
    );

    headerMain->set_subtitle(
        SUBTITLE
    );
}