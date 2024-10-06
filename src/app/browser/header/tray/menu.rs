use gtk::{
    gio::{self, SimpleAction},
    glib::{gformat, GString},
    prelude::ActionExt,
    MenuButton,
};

use std::sync::Arc;

pub struct Menu {
    gobject: MenuButton,
}
#[rustfmt::skip] // @TODO template builder?
impl Menu {
    pub fn new(
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile_directory: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Self {
        // Init model
        let model = gio::Menu::new();

            let model_tab = gio::Menu::new();
                model_tab.append(Some("New"), Some(&detailed_action_name(action_tab_append)));
                model_tab.append(Some("Pin"), Some(&detailed_action_name(action_tab_pin)));

                let model_tab_page = gio::Menu::new();

                    let model_tab_page_navigation = gio::Menu::new();
                        model_tab_page_navigation.append(Some("Base"), Some(&detailed_action_name(action_tab_page_navigation_base)));

                        let model_tab_page_navigation_history = gio::Menu::new();
                            model_tab_page_navigation_history.append(Some("Back"), Some(&detailed_action_name(action_tab_page_navigation_history_back)));
                            model_tab_page_navigation_history.append(Some("Forward"), Some(&detailed_action_name(action_tab_page_navigation_history_forward)));

                        model_tab_page_navigation.append_submenu(Some("History"), &model_tab_page_navigation_history);
                        model_tab_page_navigation.append(Some("Reload"), Some(&detailed_action_name(action_tab_page_navigation_reload)));
                         // @TODO model_tab_page_navigation.append(Some("Bookmark"), Some("win.tab_page_bookmark"));

                    model_tab_page.append_submenu(Some("Navigation"), &model_tab_page_navigation);

                model_tab.append_submenu(Some("Page"), &model_tab_page);

                let model_tab_close = gio::Menu::new();
                    model_tab_close.append(Some("Current"), Some(&detailed_action_name(action_tab_close)));
                    model_tab_close.append(Some("All"), Some(&detailed_action_name(action_tab_close_all)));

                model_tab.append_submenu(Some("Close"), &model_tab_close);

            model.append_submenu(Some("Tab"), &model_tab);

            let model_tool = gio::Menu::new();
                model_tool.append(Some("Debug"), Some(&detailed_action_name(action_tool_debug)));
                model_tool.append(Some("Profile directory"), Some(&detailed_action_name(action_tool_profile_directory)));

            model.append_submenu(Some("Tool"), &model_tool);

            model.append(Some("Quit"), Some(&detailed_action_name(action_quit)));

        // Init widget
        let gobject = MenuButton::builder().tooltip_text("Menu").build();
        gobject.set_menu_model(Some(&model));

        // Result
        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &MenuButton {
        &self.gobject
    }
}

// Private helpers
fn detailed_action_name(action: Arc<SimpleAction>) -> GString {
    gformat!("win.{}", action.name()) // @TODO find the way to ident parent group
                                      // without application-wide dependencies import
                                      // see also src/app/action.rs
}
