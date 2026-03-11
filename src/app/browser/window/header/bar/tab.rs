mod append;

use super::WindowAction;
use adw::{TabBar, TabView};
use append::Append;
use gtk::Button;
use std::rc::Rc;

pub trait Tab {
    fn tab(window_action: &Rc<WindowAction>, view: &TabView) -> Self;
}

impl Tab for TabBar {
    fn tab(window_action: &Rc<WindowAction>, view: &TabView) -> Self {
        TabBar::builder()
            .autohide(false)
            .end_action_widget(&Button::append(window_action)) // @TODO find solution to append after tabs
            .expand_tabs(false)
            .inverted(gtk::Settings::default().is_some_and(|s| {
                s.gtk_decoration_layout()
                    .is_some_and(|l| l.starts_with("close"))
            })) // show `x` button at left by respecting the env settings
            .view(view)
            .build()
    }
}
