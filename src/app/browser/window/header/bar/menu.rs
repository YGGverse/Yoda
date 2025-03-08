use super::{BrowserAction, Profile, WindowAction};
use gtk::{
    gio::{self},
    glib::{GString, Uri},
    prelude::{ActionExt, ToVariant},
    Align, MenuButton,
};
use indexmap::IndexMap;
use std::rc::Rc;

// Config options

const LABEL_MAX_LENGTH: usize = 28;
pub trait Menu {
    fn menu(action: (&Rc<BrowserAction>, &Rc<WindowAction>), profile: &Rc<Profile>) -> Self;
}

#[rustfmt::skip] // @TODO template builder?
impl Menu for MenuButton {
    // Constructors

    /// Build new `Self`
     fn menu(
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
                    for request in profile.bookmark.recent() {
                        let menu_item = gio::MenuItem::new(Some(&ellipsize(&request, LABEL_MAX_LENGTH)), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.load.simple_action.name()
                            )), Some(&request.to_variant()));

                        main_bookmarks.append_item(&menu_item);
                    } // @TODO `menu_item`

                    // Recently closed history
                    main_history_tab.remove_all();
                    for item in profile.history.memory.tab.recent() {
                        let item_request = item.page.navigation.request(); // @TODO restore entire `Item`
                        let menu_item = gio::MenuItem::new(Some(&ellipsize(&item_request, LABEL_MAX_LENGTH)), None);
                            menu_item.set_action_and_target_value(Some(&format!(
                                "{}.{}",
                                window_action.id,
                                window_action.load.simple_action.name()
                            )), Some(&item_request.to_variant()));

                            main_history_tab.append_item(&menu_item);
                    } // @TODO `menu_item`

                    // Recently visited history
                    // * in first iteration, group records by it hostname
                    // * in second iteration, collect uri path as the menu sub-item label
                    main_history_request.remove_all();

                    let mut list: IndexMap<GString, Vec<Uri>> = IndexMap::new();
                    for uri in profile.history.memory.request.recent() {
                        list.entry(match uri.host() {
                            Some(host) => host,
                            None => uri.to_str(),
                        }).or_default().push(uri);
                    }

                    for (group, items) in list {
                        let list = gio::Menu::new();

                        // Show first menu item only without children menu
                        if items.len() == 1 {
                            main_history_request.append_item(&menu_item(&window_action, &items[0], true));

                        // Create children menu items related to parental host item
                        } else {
                            for uri in items {
                                list.append_item(&menu_item(&window_action, &uri, false));
                            }
                            main_history_request.append_submenu(Some(&group), &list);
                        }
                    }
                }
            });

        menu_button
    }
}

/// Format dynamically generated strings for menu item label
/// * crop resulting string at the middle position on new `value` longer than `limit`
fn ellipsize(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        return value.to_string();
    }

    let length = (limit - 2) / 2;

    format!("{}..{}", &value[..length], &value[value.len() - length..])
}

/// Format [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// as [MenuItem](https://docs.gtk.org/gio/class.MenuItem.html) label
fn uri_to_label(uri: &Uri, is_parent: bool) -> GString {
    let path = uri.path();
    if path == "/" || path.is_empty() {
        if is_parent {
            uri.host().unwrap_or(uri.to_str())
        } else {
            gtk::glib::gformat!("{}{path}", uri.host().unwrap_or(uri.to_str()))
        }
    } else {
        path
    }
}

/// Shared helper to create new [MenuItem](https://docs.gtk.org/gio/class.MenuItem.html)
fn menu_item(action: &WindowAction, uri: &Uri, is_parent: bool) -> gio::MenuItem {
    let item = gio::MenuItem::new(
        Some(&ellipsize(&uri_to_label(uri, is_parent), LABEL_MAX_LENGTH)),
        None,
    );
    item.set_action_and_target_value(
        Some(&format!(
            "{}.{}",
            action.id,
            action.load.simple_action.name()
        )),
        Some(&uri.to_string().to_variant()),
    );
    item
}
