mod control;
mod menu;
mod tab;

use control::Control;
use menu::Menu;
use tab::Tab;

use super::{BrowserAction, Profile, WindowAction};
use adw::TabView;
use gtk::{prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub trait Bar {
    fn bar(
        action: (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
        view: &TabView,
    ) -> Self;
}

impl Bar for Box {
    // Constructors

    /// Build new `Self`
    fn bar(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
        view: &TabView,
    ) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        g_box.append(&Tab::new(window_action, view).widget.tab_bar);
        g_box.append(&Menu::build((browser_action, window_action), profile).menu_button);
        g_box.append(&Control::new().window_controls);
        g_box
    }
}
