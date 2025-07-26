use super::{BrowserAction, WindowAction};
use gtk::{
    Align, MenuButton,
    gio::{self},
    prelude::ActionExt,
};
use std::rc::Rc;

// Config options

pub trait Menu {
    fn menu(actions: (&Rc<BrowserAction>, &Rc<WindowAction>)) -> Self;
}

#[rustfmt::skip] // @TODO template builder?
impl Menu for MenuButton {
    // Constructors

    /// Build new `Self`
     fn menu(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
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

                main_page.append(Some("Find.."), Some(&format!(
                    "{}.{}",
                    window_action.id,
                    window_action.find.simple_action.name()
                )));

                // Main > Page > File
                let main_page_file = gio::Menu::new();

                    main_page_file.append(Some("Open.."), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.open.simple_action.name()
                    )));

                    main_page_file.append(Some("Save as.."), Some(&format!(
                        "{}.{}",
                        window_action.id,
                        window_action.save_as.simple_action.name()
                    )));

                main_page.append_submenu(Some("File"), &main_page_file);

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

                    main_page_navigation.append_submenu(Some("Navigation"), &main_page_navigation_history);

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

            // Main > Bookmarks
            main.append(Some("Bookmarks"), Some(&format!(
                "{}.{}",
                browser_action.id,
                browser_action.bookmarks.simple_action.name()
            )));

            // Main > History
            main.append(Some("History"), Some(&format!(
                "{}.{}",
                browser_action.id,
                browser_action.history.simple_action.name()
            )));

            // Main > Settings
            let main_settings = gio::Menu::new();

                // Main > Settings > Proxy connection
                main_settings.append(Some("Proxy connection"), Some(&format!(
                    "{}.{}",
                    browser_action.id,
                    browser_action.proxy.simple_action.name()
                )));

            main.append_submenu(Some("Settings"), &main_settings);

            // Main > Tool
            let main_tool = gio::Menu::new();

                // Debug
                main_tool.append(Some("Debug"), Some(&format!(
                    "{}.{}",
                    browser_action.id,
                    browser_action.debug.simple_action.name()
                )));

                main_tool.append(Some("Open profile directory"), Some(&format!(
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

        // Init main widget
        MenuButton::builder()
            .css_classes(["flat"])
            .icon_name("open-menu-symbolic")
            .menu_model(&main)
            .tooltip_text("Menu")
            .valign(Align::Center)
            .build()
    }
}
