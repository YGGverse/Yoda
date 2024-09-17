#include "menu.hpp"

using namespace app::browser::header;

Menu::Menu(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__DEBUG,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__QUIT,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_CLEAN,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_RESTORE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__SESSION_SAVE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_APPEND,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PIN,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_CLOSE_ALL,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__TAB_PAGE_NAVIGATION_RELOAD
) {
    // Init widget
    set_tooltip_text(
        _("Menu")
    );

    // Init components @TODO make builder
    const auto MENU = Gio::Menu::create();

        const auto MENU__SESSION = Gio::Menu::create();

            MENU__SESSION->append(
                _("Clean"),
                get_action_detailed_name(
                    ACTION__SESSION_CLEAN
                )
            );

            MENU__SESSION->append(
                _("Restore"),
                get_action_detailed_name(
                    ACTION__SESSION_RESTORE
                )
            );

            MENU__SESSION->append(
                _("Save"),
                get_action_detailed_name(
                    ACTION__SESSION_SAVE
                )
            );

        MENU->append_submenu(
            _("Session"),
            MENU__SESSION
        );

        const auto MENU__TAB = Gio::Menu::create();

            MENU__TAB->append(
                _("New.."),
                get_action_detailed_name(
                    ACTION__TAB_APPEND
                )
            );

            MENU__TAB->append(
                _("Pin"),
                get_action_detailed_name(
                    ACTION__TAB_PIN
                )
            );

                const auto MENU__TAB_PAGE = Gio::Menu::create();

                    const auto MENU__TAB_PAGE_NAVIGATION = Gio::Menu::create();

                        const auto MENU__TAB_PAGE_NAVIGATION_HISTORY = Gio::Menu::create();

                            MENU__TAB_PAGE_NAVIGATION_HISTORY->append(
                                _("Back"),
                                get_action_detailed_name(
                                    ACTION__TAB_PAGE_NAVIGATION_HISTORY_BACK
                                )
                            );

                            MENU__TAB_PAGE_NAVIGATION_HISTORY->append(
                                _("Forward"),
                                get_action_detailed_name(
                                    ACTION__TAB_PAGE_NAVIGATION_HISTORY_FORWARD
                                )
                            );

                        MENU__TAB_PAGE_NAVIGATION->append_submenu(
                            _("History"),
                            MENU__TAB_PAGE_NAVIGATION_HISTORY
                        );

                        MENU__TAB_PAGE_NAVIGATION->append(
                            _("Reload"),
                            get_action_detailed_name(
                                ACTION__TAB_PAGE_NAVIGATION_RELOAD
                            )
                        );

                    MENU__TAB_PAGE->append_submenu(
                        _("Navigation"),
                        MENU__TAB_PAGE_NAVIGATION
                    );

            MENU__TAB->append_submenu(
                _("Page"),
                MENU__TAB_PAGE
            );

            const auto MENU__TAB_CLOSE = Gio::Menu::create();

                MENU__TAB_CLOSE->append(
                    _("Active tab"),
                    get_action_detailed_name(
                        ACTION__TAB_CLOSE
                    )
                );

                // @TODO
                /*
                MENU__TAB_CLOSE->append(
                    _("All tabs to left"),
                    get_action_detailed_name(
                        ACTION__TAB_CLOSE_LEFT
                    )
                );

                MENU__TAB_CLOSE->append(
                    _("All tabs to right"),
                    get_action_detailed_name(
                        ACTION__TAB_CLOSE_RIGHT
                    )
                );
                */
                MENU__TAB_CLOSE->append(
                    _("All tabs"),
                    get_action_detailed_name(
                        ACTION__TAB_CLOSE_ALL
                    )
                );

            MENU__TAB->append_submenu(
                _("Close"),
                MENU__TAB_CLOSE
            );

        MENU->append_submenu(
            _("Tab"),
            MENU__TAB
        );

        const auto MENU__TOOL = Gio::Menu::create();

            MENU__TOOL->append(
                _("Debug"),
                get_action_detailed_name(
                    ACTION__DEBUG
                )
            );

        MENU->append_submenu(
            _("Tool"),
            MENU__TOOL
        );

        MENU->append(
            _("Quit"),
            get_action_detailed_name(
                ACTION__QUIT
            )
        );

    set_menu_model(
        MENU
    );
}

Glib::ustring Menu::get_action_detailed_name(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION
) {
    return Glib::ustring::sprintf(
        "win.%s", ACTION->get_name()
    );
}