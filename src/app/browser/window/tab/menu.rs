use std::rc::Rc;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::{gio::SimpleAction, prelude::ActionExt};

/// Context menu wrapper
///
/// https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/method.TabView.get_menu_model.html
pub struct Menu {
    gobject: gtk::gio::Menu,
}

impl Menu {
    // Constructors

    /// Create new `Self`
    pub fn new(
        window_action: Rc<WindowAction>,
        action_page_close_all: SimpleAction,
        action_page_close: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_home: SimpleAction,
    ) -> Self {
        let main = gtk::gio::Menu::new();

        main.append(
            Some("Reload"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.reload().id()
            )),
        );

        main.append(
            Some("Pin"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.pin().id()
            )),
        );

        let navigation = gtk::gio::Menu::new();

        navigation.append(Some("Home"), Some(&detailed_action_name(action_page_home)));

        main.append_section(None, &navigation);

        let history = gtk::gio::Menu::new();

        history.append(
            Some("Back"),
            Some(&detailed_action_name(action_page_history_back)),
        );

        history.append(
            Some("Forward"),
            Some(&detailed_action_name(action_page_history_forward)),
        );

        main.append_submenu(Some("History"), &history);

        let close = gtk::gio::Menu::new();

        close.append(
            Some("Current"),
            Some(&detailed_action_name(action_page_close)),
        );

        close.append(
            Some("All"),
            Some(&detailed_action_name(action_page_close_all)),
        );

        main.append_submenu(Some("Close"), &close);

        Self { gobject: main }
    }

    /// Get reference to [Menu](https://docs.gtk.org/gio/class.Menu.html) `GObject`
    pub fn gobject(&self) -> &gtk::gio::Menu {
        &self.gobject
    }
}

// Private helpers

fn detailed_action_name(action: SimpleAction) -> String {
    format!("win.{}", action.name()) // @TODO find the way to ident parent group
                                     // without application-wide dependencies import
                                     // see also src/app/action.rs
}
