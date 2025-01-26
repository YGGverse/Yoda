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
            .expand_tabs(false)
            .end_action_widget(&Button::append(window_action)) // @TODO find solution to append after tabs
            .view(view)
            .build()
    }
}
