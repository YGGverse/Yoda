use crate::app::browser::window::action::Action as WindowAction;
use gtk::prelude::ActionExt;
use std::rc::Rc;

/// Context menu wrapper
///
/// https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/method.TabView.get_menu_model.html
pub struct Menu {
    pub gobject: gtk::gio::Menu,
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
                window_action.reload.gobject.name()
            )),
        );

        main.append(
            Some("Save as.."),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.save_as.gobject.name()
            )),
        );

        let main_mark = gtk::gio::Menu::new();

        main_mark.append(
            Some("Bookmark"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.bookmark.gobject.name()
            )),
        );

        main_mark.append(
            Some("Pin"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.pin.gobject.name()
            )),
        );

        main.append_section(None, &main_mark);

        let main_tools = gtk::gio::Menu::new();

        main_tools.append(
            Some("Source"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.source.gobject.name()
            )),
        );

        main.append_section(None, &main_tools);

        let navigation = gtk::gio::Menu::new();

        navigation.append(
            Some("Home"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.home.gobject.name()
            )),
        );

        main.append_section(None, &navigation);

        let history = gtk::gio::Menu::new();

        history.append(
            Some("Back"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.history_back.gobject.name()
            )),
        );

        history.append(
            Some("Forward"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.history_forward.gobject.name()
            )),
        );

        main.append_submenu(Some("History"), &history);

        let close = gtk::gio::Menu::new();

        close.append(
            Some("Current"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.close.gobject.name()
            )),
        );

        close.append(
            Some("All"),
            Some(&format!(
                "{}.{}",
                window_action.id,
                window_action.close_all.gobject.name()
            )),
        );

        main.append_submenu(Some("Close"), &close);

        Self { gobject: main }
    }
}
