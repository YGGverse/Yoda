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
        action_tool_debug: SimpleAction,
        action_tool_profile: SimpleAction,
        action_quit: SimpleAction,
        action_tab_append: SimpleAction,
        action_tab_close: SimpleAction,
        action_tab_close_all: SimpleAction,
        action_tab_page_navigation_base: SimpleAction,
        action_tab_page_navigation_history_back: SimpleAction,
        action_tab_page_navigation_history_forward: SimpleAction,
        action_tab_page_navigation_reload: SimpleAction,
        action_tab_pin: SimpleAction,
    ) -> Arc<Self> {
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
                model_tool.append(Some("Profile"), Some(&detailed_action_name(action_tool_profile)));

            model.append_submenu(Some("Tool"), &model_tool);

            model.append(Some("Quit"), Some(&detailed_action_name(action_quit)));

        // Result
        Arc::new(Self { widget:Widget::new_arc(&model) })
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
