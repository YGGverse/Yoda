mod control;
mod menu;
mod tab;

use control::Control;
use menu::Menu;
use tab::Tab;

use super::{BrowserAction, Profile, WindowAction};
use adw::{TabBar, TabView};
use gtk::{Box, MenuButton, Orientation, prelude::BoxExt};
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

        g_box.append(&TabBar::tab(window_action, view));
        g_box.append(&MenuButton::menu((browser_action, window_action), profile));
        g_box.append(&Control::new().window_controls);
        g_box
    }
}
