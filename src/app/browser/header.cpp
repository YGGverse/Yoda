#include "header.hpp"
#include "header/main.hpp"
#include "header/menu.hpp"
#include "header/tab.hpp"

using namespace app::browser;

Header::Header(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__RESTORE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SAVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_APPEND,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
) {
    // Init widget
    set_show_title_buttons(
        SHOW_TITLE_BUTTONS
    );

    // Init components
    headerMenu = Gtk::make_managed<header::Menu>(
        ACTION__DEBUG,
        ACTION__QUIT,
        ACTION__RESTORE,
        ACTION__SAVE,
        ACTION__MAIN_TAB_APPEND,
        ACTION__MAIN_TAB_CLOSE_ACTIVE,
        ACTION__MAIN_TAB_CLOSE_ALL,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
    );

        pack_start(
            * headerMenu
        );

    headerTab = Gtk::make_managed<header::Tab>();

        pack_start(
            * headerTab
        );

    headerMain = Gtk::make_managed<header::Main>();

        set_title_widget(
            * headerMain
        );
}

void Header::refresh(
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