use gtk::{
    gio::{self, SimpleAction},
    glib::{gformat, GString},
    prelude::ActionExt,
    MenuButton,
};

use std::sync::Arc;

pub struct Menu {
    widget: MenuButton,
}

impl Menu {
    pub fn new(
        action_debug: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Self {
        // Init model
        let model_tab = gio::Menu::new();
        model_tab.append(Some("New"), Some(&detailed_action_name(action_tab_append)));
        model_tab.append(Some("Pin"), Some(&detailed_action_name(action_tab_pin)));

        let model_tab_page = gio::Menu::new();
        model_tab_page.append(Some("Base"), Some("win.tab_page_base")); // @TODO

        let model_tab_page_history = gio::Menu::new();
        model_tab_page_history.append(Some("Back"), Some("win.tab_page_history_back")); // @TODO
        model_tab_page_history.append(Some("Forward"), Some("win.tab_page_history_forward")); // @TODO
        model_tab_page.append_submenu(Some("History"), &model_tab_page_history);

        model_tab_page.append(
            Some("Reload"),
            Some(&detailed_action_name(action_tab_page_reload)),
        );
        model_tab_page.append(Some("Bookmark"), Some("win.tab_page_bookmark")); // @TODO

        model_tab.append_submenu(Some("Page"), &model_tab_page);

        let model_tab_close = gio::Menu::new();
        model_tab_close.append(
            Some("Current"),
            Some(&detailed_action_name(action_tab_close)),
        );
        model_tab_close.append(
            Some("All"),
            Some(&detailed_action_name(action_tab_close_all)),
        );
        model_tab.append_submenu(Some("Close"), &model_tab_close);

        let model = gio::Menu::new();

        model.append_submenu(Some("Tab"), &model_tab);
        model.append(Some("Debug"), Some(&detailed_action_name(action_debug)));
        model.append(Some("Quit"), Some(&detailed_action_name(action_quit)));

        // Init widget
        let widget = MenuButton::builder().tooltip_text("Menu").build();
        widget.set_menu_model(Some(&model));

        // Result
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &MenuButton {
        &self.widget
    }
}

// Private helpers
fn detailed_action_name(action: Arc<SimpleAction>) -> GString {
    gformat!("win.{}", action.name()) // @TODO find the way to ident parent group
                                      // without application-wide dependencies import
}
