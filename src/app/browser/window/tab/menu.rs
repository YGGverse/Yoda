use crate::app::browser::window::action::Action as WindowAction;
use gtk::prelude::ActionExt;
use std::rc::Rc;

/// Context menu wrapper
///
/// https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/method.TabView.get_menu_model.html
pub struct Menu {
    pub main: gtk::gio::Menu,
}

impl Menu {
    // Constructors

    /// Create new `Self`
    pub fn new(window_action: Rc<WindowAction>) -> Self {
        let main = gtk::gio::Menu::new();

        main.append(
            Some("Reload"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.reload.simple_action.name()
            )),
        );

        main.append(
            Some("Find.."),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.find.simple_action.name()
            )),
        );

        main.append(
            Some("Save as.."),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.save_as.simple_action.name()
            )),
        );

        let main_mark = gtk::gio::Menu::new();

        main_mark.append(
            Some("Bookmark"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.bookmark.simple_action.name()
            )),
        );

        main_mark.append(
            Some("Pin"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.pin.simple_action.name()
            )),
        );

        main.append_section(None, &main_mark);

        let main_tools = gtk::gio::Menu::new();

        main_tools.append(
            Some("Source"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.source.simple_action.name()
            )),
        );

        main.append_section(None, &main_tools);

        let navigation = gtk::gio::Menu::new();

        navigation.append(
            Some("Home"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.home.simple_action.name()
            )),
        );

        main.append_section(None, &navigation);

        let history = gtk::gio::Menu::new();

        history.append(
            Some("Back"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.history_back.simple_action.name()
            )),
        );

        history.append(
            Some("Forward"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.history_forward.simple_action.name()
            )),
        );

        main.append_submenu(Some("History"), &history);

        let close = gtk::gio::Menu::new();

        close.append(
            Some("Current"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.close.simple_action.name()
            )),
        );

        close.append(
            Some("All"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.close_all.simple_action.name()
            )),
        );

        main.append_submenu(Some("Close"), &close);

        Self { main }
    }
}
