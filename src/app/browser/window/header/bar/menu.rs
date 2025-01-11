use super::{BrowserAction, Profile, WindowAction};
use gtk::{
    gio::{self},
    prelude::{ActionExt, ToVariant},
    Align, MenuButton,
};
use std::rc::Rc;

// Config options

const RECENT_BOOKMARKS: usize = 50;

pub struct Menu {
    pub menu_button: MenuButton,
}

#[rustfmt::skip] // @TODO template builder?
impl Menu {
    pub fn new(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
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

            // Main > Bookmark
            // * menu items dynamically generated using profile memory pool and `set_create_popup_func`
            let main_bookmarks = gio::Menu::new();

                main.append_submenu(Some("Bookmarks"), &main_bookmarks);

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

        // Init main widget
        let menu_button = MenuButton::builder()
                .css_classes(["flat"])
                .icon_name("open-menu-symbolic")
                .menu_model(&main)
                .tooltip_text("Menu")
                .valign(Align::Center)
                .build();

            // Generate dynamical menu items
            menu_button.set_create_popup_func({
                let profile = profile.clone();
                let main_bookmarks = main_bookmarks.clone();
                let window_action = window_action.clone();
                move |_| {
                    main_bookmarks.remove_all();
                    for bookmark in profile.bookmark.memory.recent(RECENT_BOOKMARKS) {
                        let menu_item = gio::MenuItem::new(Some(&bookmark), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.open.simple_action.name()
                            )), Some(&bookmark.to_variant()));

                        main_bookmarks.append_item(&menu_item);
                    }
                    // Show all bookmarks menu item
                    // if profile.bookmark.memory.total() > RECENT_BOOKMARKS {
                    // @TODO
                    // }
                }
            });

        // Result
        Self {
            menu_button
        }
    }
}
