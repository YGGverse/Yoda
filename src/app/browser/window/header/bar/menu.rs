mod widget;

use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use gtk::{
    gio::{self, SimpleAction},
    prelude::ActionExt,
    MenuButton,
};
use std::rc::Rc;

pub struct Menu {
    widget: Rc<Widget>,
}
#[rustfmt::skip] // @TODO template builder?
impl Menu {
    pub fn new_rc(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
    ) -> Rc<Self> {
        // Main
        let main = gio::Menu::new();

            // Main > Page
            let main_page = gio::Menu::new();
                main_page.append(Some("New"), Some(&format!(
                    "{}.{}",
                    window_action.id(),
                    window_action.append().id()
                )));

                main_page.append(Some("Reload"), Some(&format!(
                    "{}.{}",
                    window_action.id(),
                    window_action.reload().id()
                )));

                main_page.append(Some("Pin"), Some(&format!(
                    "{}.{}",
                    window_action.id(),
                    window_action.pin().id()
                )));

                // Main > Page > Navigation
                let main_page_navigation = gio::Menu::new();
                    main_page_navigation.append(Some("Home"), Some(&detailed_action_name(&action_page_home)));

                    // Main > Page > Navigation > History
                    let main_page_navigation_history = gio::Menu::new();
                        main_page_navigation_history.append(Some("Back"), Some(&detailed_action_name(&action_page_history_back)));
                        main_page_navigation_history.append(Some("Forward"), Some(&detailed_action_name(&action_page_history_forward)));

                    main_page_navigation.append_submenu(Some("History"), &main_page_navigation_history);

                main_page.append_section(None, &main_page_navigation);

                // Main > Page > Close
                let main_page_close = gio::Menu::new();
                    main_page_close.append(Some("Current"), Some(&detailed_action_name(&action_page_close)));
                    main_page_close.append(Some("All"), Some(&detailed_action_name(&action_page_close_all)));

                    main_page.append_submenu(Some("Close"), &main_page_close);

                main.append_submenu(Some("Page"), &main_page);

            // Main > Tool
            let main_tool = gio::Menu::new();

                // Debug
                main_tool.append(Some("Debug"), Some(&format!(
                    "{}.{}",
                    browser_action.id(),
                    browser_action.debug().id()
                )));

                main_tool.append(Some("Profile"), Some(&format!(
                    "{}.{}",
                    browser_action.id(),
                    browser_action.profile().id()
                )));

                main_tool.append(Some("About"), Some(&format!(
                    "{}.{}",
                    browser_action.id(),
                    browser_action.about().id()
                )));

        main.append_submenu(Some("Tool"), &main_tool);

        main.append(Some("Quit"), Some(&format!(
            "{}.{}",
            browser_action.id(),
            browser_action.close().id()
        )));

        // Result
        Rc::new(Self { widget:Widget::new_rc(&main) })
    }

    // Getters
    pub fn gobject(&self) -> &MenuButton {
        self.widget.gobject()
    }
}

// Private helpers
fn detailed_action_name(action: &SimpleAction) -> String {
    format!("win.{}", action.name()) // @TODO find the way to ident parent group
                                     // without application-wide dependencies import
                                     // see also src/app/action.rs
}
