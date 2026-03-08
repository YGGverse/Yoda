mod control;
mod menu;
mod tab;

use control::Control;
use menu::Menu;
use tab::Tab;

use super::{BrowserAction, WindowAction};
use adw::{TabBar, TabView};
use gtk::{Box, MenuButton, Orientation, prelude::BoxExt};
use std::rc::Rc;

pub trait Bar {
    fn bar(action: (&Rc<BrowserAction>, &Rc<WindowAction>), view: &TabView) -> Self;
}

impl Bar for Box {
    // Constructors

    /// Build new `Self`
    fn bar(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        view: &TabView,
    ) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();
        // left controls placement
        if gtk::Settings::default().is_some_and(|s| {
            s.gtk_decoration_layout()
                .is_some_and(|l| l.starts_with("close"))
        }) {
            g_box.append(&Control::left().window_controls);
            g_box.append(&MenuButton::menu((browser_action, window_action)));
            g_box.append(&TabBar::tab(window_action, view))
        // default layout
        } else {
            g_box.append(&TabBar::tab(window_action, view));
            g_box.append(&MenuButton::menu((browser_action, window_action)));
            g_box.append(&Control::right().window_controls)
        }
        g_box
    }
}
