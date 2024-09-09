#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_APPEND,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ACTIVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_SESSION_RESTORE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__MAIN_TAB_SESSION_SAVE
) {
    // Init widget
    set_tooltip_text(
        _("Menu")
    );

    // Init components @TODO make builder
    const auto MENU__MAIN = Gio::Menu::create();

        const auto MENU__MAIN_TAB = Gio::Menu::create();

            MENU__MAIN_TAB->append(
                _("New.."),
                get_action_detailed_name(
                    ACTION__MAIN_TAB_APPEND
                )
            );

                const auto MENU__MAIN_TAB_PAGE = Gio::Menu::create();

                    const auto MENU__MAIN_TAB_PAGE_NAVIGATION = Gio::Menu::create();

                        const auto MENU__MAIN_TAB_PAGE_NAVIGATION_HISTORY = Gio::Menu::create();

                            MENU__MAIN_TAB_PAGE_NAVIGATION_HISTORY->append(
                                _("Back"),
                                get_action_detailed_name(
                                    ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_BACK
                                )
                            );

                            MENU__MAIN_TAB_PAGE_NAVIGATION_HISTORY->append(
                                _("Forward"),
                                get_action_detailed_name(
                                    ACTION__MAIN_TAB_PAGE_NAVIGATION_HISTORY_FORWARD
                                )
                            );

                        MENU__MAIN_TAB_PAGE_NAVIGATION->append_submenu(
                            _("History"),
                            MENU__MAIN_TAB_PAGE_NAVIGATION_HISTORY
                        );

                        MENU__MAIN_TAB_PAGE_NAVIGATION->append(
                            _("Update"),
                            get_action_detailed_name(
                                ACTION__MAIN_TAB_PAGE_NAVIGATION_UPDATE
                            )
                        );

                    MENU__MAIN_TAB_PAGE->append_submenu(
                        _("Navigation"),
                        MENU__MAIN_TAB_PAGE_NAVIGATION
                    );

            MENU__MAIN_TAB->append_submenu(
                _("Page"),
                MENU__MAIN_TAB_PAGE
            );

                const auto MENU__MAIN_TAB_SESSION = Gio::Menu::create();

                    MENU__MAIN_TAB_SESSION->append(
                        _("Restore"),
                        get_action_detailed_name(
                            ACTION__MAIN_TAB_SESSION_RESTORE
                        )
                    );

                    MENU__MAIN_TAB_SESSION->append(
                        _("Save"),
                        get_action_detailed_name(
                            ACTION__MAIN_TAB_SESSION_SAVE
                        )
                    );

                MENU__MAIN_TAB->append_submenu(
                    _("Session"),
                    MENU__MAIN_TAB_SESSION
                );

            const auto MENU__MAIN_TAB_CLOSE = Gio::Menu::create();

                MENU__MAIN_TAB_CLOSE->append(
                    _("Active tab"),
                    get_action_detailed_name(
                        ACTION__MAIN_TAB_CLOSE_ACTIVE
                    )
                );

                // @TODO
                /*
                MENU__MAIN_TAB_CLOSE->append(
                    _("All tabs to left"),
                    get_action_detailed_name(
                        ACTION__MAIN_TAB_CLOSE_LEFT
                    )
                );

                MENU__MAIN_TAB_CLOSE->append(
                    _("All tabs to right"),
                    get_action_detailed_name(
                        ACTION__MAIN_TAB_CLOSE_RIGHT
                    )
                );
                */
                MENU__MAIN_TAB_CLOSE->append(
                    _("All tabs"),
                    get_action_detailed_name(
                        ACTION__MAIN_TAB_CLOSE_ALL
                    )
                );

            MENU__MAIN_TAB->append_submenu(
                _("Close"),
                MENU__MAIN_TAB_CLOSE
            );

        MENU__MAIN->append_submenu(
            _("Tab"),
            MENU__MAIN_TAB
        );

        const auto MENU__MAIN_TOOLS = Gio::Menu::create();

            MENU__MAIN_TOOLS->append(
                _("Debug"),
                get_action_detailed_name(
                    ACTION__DEBUG
                )
            );

        MENU__MAIN->append_submenu(
            _("Tools"),
            MENU__MAIN_TOOLS
        );

        MENU__MAIN->append(
            _("Quit"),
            get_action_detailed_name(
                ACTION__QUIT
            )
        );

    set_menu_model(
        MENU__MAIN
    );
}

Glib::ustring Menu::get_action_detailed_name(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION
) {
    return Glib::ustring::sprintf(
        "win.%s", ACTION->get_name()
    );
}