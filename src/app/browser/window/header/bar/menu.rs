mod widget;

use widget::Widget;

use gtk::{
    gio::{self, SimpleAction},
    glib::{gformat, GString},
    prelude::ActionExt,
    MenuButton,
};

use std::sync::Arc;

pub struct Menu {
    widget: Arc<Widget>,
}
#[rustfmt::skip] // @TODO template builder?
impl Menu {
    pub fn new_arc(
        action_about: SimpleAction,
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
    ) -> Arc<Self> {
        // Main
        let main = gio::Menu::new();

            // Main > Page
            let main_page = gio::Menu::new();
                main_page.append(Some("New"), Some(&detailed_action_name(action_page_new)));
                main_page.append(Some("Reload"), Some(&detailed_action_name(action_page_reload)));
                main_page.append(Some("Pin"), Some(&detailed_action_name(action_page_pin)));

                // Main > Page > Navigation
                let main_page_navigation = gio::Menu::new();
                    main_page_navigation.append(Some("Home"), Some(&detailed_action_name(action_page_home)));

                    // Main > Page > Navigation > History
                    let main_page_navigation_history = gio::Menu::new();
                        main_page_navigation_history.append(Some("Back"), Some(&detailed_action_name(action_page_history_back)));
                        main_page_navigation_history.append(Some("Forward"), Some(&detailed_action_name(action_page_history_forward)));

                    main_page_navigation.append_submenu(Some("History"), &main_page_navigation_history);

                main_page.append_section(None, &main_page_navigation);

                // Main > Page > Close
                let main_page_close = gio::Menu::new();
                    main_page_close.append(Some("Current"), Some(&detailed_action_name(action_page_close)));
                    main_page_close.append(Some("All"), Some(&detailed_action_name(action_page_close_all)));

                    main_page.append_submenu(Some("Close"), &main_page_close);

                main.append_submenu(Some("Page"), &main_page);

            // Main > Tool
            let main_tool = gio::Menu::new();
                main_tool.append(Some("Debug"), Some(&detailed_action_name(action_debug)));
                main_tool.append(Some("Profile"), Some(&detailed_action_name(action_profile)));
                main_tool.append(Some("About"), Some(&detailed_action_name(action_about)));

            main.append_submenu(Some("Tool"), &main_tool);

            main.append(Some("Quit"), Some(&detailed_action_name(action_quit)));

        // Result
        Arc::new(Self { widget:Widget::new_arc(&main) })
    }

    // Getters
    pub fn gobject(&self) -> &MenuButton {
        &self.widget.gobject()
    }
}

// Private helpers
fn detailed_action_name(action: SimpleAction) -> GString {
    gformat!("win.{}", action.name()) // @TODO find the way to ident parent group
                                      // without application-wide dependencies import
                                      // see also src/app/action.rs
}
