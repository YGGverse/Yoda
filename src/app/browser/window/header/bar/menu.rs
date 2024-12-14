mod widget;

use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use gtk::{
    gio::{self},
    prelude::ActionExt,
};
use std::rc::Rc;

pub struct Menu {
    pub widget: Rc<Widget>,
}
#[rustfmt::skip] // @TODO template builder?
impl Menu {
    pub fn new(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
    ) -> Self {
        // Main
        let main = gio::Menu::new();

            // Main > Page
            let main_page = gio::Menu::new();
                main_page.append(Some("New"), Some(&format!(
                    "{}.{}",
                    window_action.id,
                    window_action.append.simple_action.name()
                )));

                main_page.append(Some("Reload"), Some(&format!(
                    "{}.{}",
                    window_action.id,
                    window_action.reload.simple_action.name()
                )));

                main_page.append(Some("Save as.."), Some(&format!(
                    "{}.{}",
                    window_action.id,
                    window_action.save_as.simple_action.name()
                )));

                // Main > Page > Mark
                let main_page_mark = gio::Menu::new();

                    main_page_mark.append(Some("Bookmark"), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.bookmark.simple_action.name()
                    )));

                    main_page_mark.append(Some("Pin"), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.pin.simple_action.name()
                    )));

                main_page.append_section(None, &main_page_mark);

                // Main > Page > Tools
                let main_page_tools = gio::Menu::new();

                main_page_tools.append(Some("Source"), Some(&format!(
                    "{}.{}",
                    window_action.id,
                    window_action.source.simple_action.name()
                )));

                main_page.append_section(None, &main_page_tools);

                // Main > Page > Navigation
                let main_page_navigation = gio::Menu::new();

                    main_page_navigation.append(Some("Home"), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.home.simple_action.name()
                    )));

                    // Main > Page > Navigation > History
                    let main_page_navigation_history = gio::Menu::new();

                        main_page_navigation_history.append(Some("Back"), Some(&format!(
                            "{}.{}",
                            window_action.id,
                            window_action.history_back.simple_action.name()
                        )));

                        main_page_navigation_history.append(Some("Forward"), Some(&format!(
                            "{}.{}",
                            window_action.id,
                            window_action.history_forward.simple_action.name()
                        )));

                    main_page_navigation.append_submenu(Some("History"), &main_page_navigation_history);

                main_page.append_section(None, &main_page_navigation);

                // Main > Page > Close
                let main_page_close = gio::Menu::new();

                    main_page_close.append(Some("Current"), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.close.simple_action.name()
                    )));

                    main_page_close.append(Some("All"), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.close_all.simple_action.name()
                    )));

                    main_page.append_submenu(Some("Close"), &main_page_close);

                main.append_submenu(Some("Page"), &main_page);

            // Main > Tool
            let main_tool = gio::Menu::new();

                // Debug
                main_tool.append(Some("Debug"), Some(&format!(
                    "{}.{}",
                    browser_action.id,
                    browser_action.debug.simple_action.name()
                )));

                main_tool.append(Some("Profile"), Some(&format!(
                    "{}.{}",
                    browser_action.id,
                    browser_action.profile.simple_action.name()
                )));

                main_tool.append(Some("About"), Some(&format!(
                    "{}.{}",
                    browser_action.id,
                    browser_action.about.simple_action.name()
                )));

        main.append_submenu(Some("Tool"), &main_tool);

        main.append(Some("Quit"), Some(&format!(
            "{}.{}",
            browser_action.id,
            browser_action.close.simple_action.name()
        )));

        // Result
        Self { widget:Rc::new(Widget::new(&main)) }
    }
}
