use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

/// Context menu wrapper
///
/// https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/method.TabView.get_menu_model.html
pub struct Menu {
    gobject: gtk::gio::Menu,
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

        navigation.append(
            Some("Home"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.home().id()
            )),
        );

        main.append_section(None, &navigation);

        let history = gtk::gio::Menu::new();

        history.append(
            Some("Back"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.history_back().id()
            )),
        );

        history.append(
            Some("Forward"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.history_forward().id()
            )),
        );

        main.append_submenu(Some("History"), &history);

        let close = gtk::gio::Menu::new();

        close.append(
            Some("Current"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.close().id()
            )),
        );

        close.append(
            Some("All"),
            Some(&format!(
                "{}.{}",
                window_action.id(),
                window_action.close_all().id()
            )),
        );

        main.append_submenu(Some("Close"), &close);

        Self { gobject: main }
    }

    /// Get reference to [Menu](https://docs.gtk.org/gio/class.Menu.html) `GObject`
    pub fn gobject(&self) -> &gtk::gio::Menu {
        &self.gobject
    }
}
