use super::{BrowserAction, Profile, WindowAction};
use gtk::{
    gio::{self},
    prelude::{ActionExt, EditableExt, ToVariant},
    Align, MenuButton,
};
use std::rc::Rc;

// Config options

const LABEL_MAX_LENGTH: usize = 32;
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

                    main_page_navigation.append_submenu(Some("Navigation history"), &main_page_navigation_history);

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

            // Main > History
            let main_history = gio::Menu::new();

                // Main > History > Recently closed
                // * menu items dynamically generated using profile memory pool and `set_create_popup_func`
                let main_history_tab = gio::Menu::new();
                    main_history.append_submenu(Some("Recently closed"), &main_history_tab);

                // Main > History > Recent requests
                // * menu items dynamically generated using profile memory pool and `set_create_popup_func`
                let main_history_request = gio::Menu::new();
                    main_history.append_section(None, &main_history_request);

                main.append_submenu(Some("History"), &main_history);

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
                    // Bookmarks
                    main_bookmarks.remove_all();
                    for request in profile.bookmark.memory.recent() {
                        let menu_item = gio::MenuItem::new(Some(&label(&request, LABEL_MAX_LENGTH)), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.open.simple_action.name()
                            )), Some(&request.to_variant()));

                        main_bookmarks.append_item(&menu_item);
                    }
                    // Show all bookmarks menu item
                    // if profile.bookmark.memory.total() > RECENT_BOOKMARKS {
                    // @TODO
                    // }

                    // Recently closed history
                    main_history_tab.remove_all();
                    for item in profile.history.memory.tab.recent() {
                        let item_request = item.page.navigation.request.widget.entry.text(); // @TODO restore entire `Item`
                        let menu_item = gio::MenuItem::new(Some(&label(&item_request, LABEL_MAX_LENGTH)), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.open.simple_action.name()
                            )), Some(&item_request.to_variant()));

                            main_history_tab.append_item(&menu_item);
                    }

                    // Recently visited history
                    main_history_request.remove_all();
                    for request in profile.history.memory.request.recent() {
                        let menu_item = gio::MenuItem::new(Some(&label(&request, LABEL_MAX_LENGTH)), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.open.simple_action.name()
                            )), Some(&request.to_variant()));

                            main_history_request.append_item(&menu_item);
                    }
                }
            });

        // Result
        Self {
            menu_button
        }
    }
}

/// Format dynamically generated strings for menu item labels
/// * trim gemini scheme prefix
/// * trim slash postfix
/// * crop resulting string at the middle position on new `value` longer than `limit`
fn label(value: &str, limit: usize) -> String {
    let value = value.trim_start_matches("gemini://");
    let value = value.trim_end_matches('/');

    if value.len() <= limit {
        return value.to_string();
    }

    let length = (limit - 2) / 2;

    format!("{}..{}", &value[..length], &value[value.len() - length..])
}
