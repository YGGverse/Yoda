mod bar;

use super::{Action as WindowAction, BrowserAction};
use adw::{TabView, ToolbarView};
use bar::Bar;
use gtk::Box;
use std::rc::Rc;

pub trait Header {
    fn header(action: (&Rc<BrowserAction>, &Rc<WindowAction>), tab_view: &TabView) -> Self;
}

impl Header for ToolbarView {
    // Constructors

    /// Build new `Self`
    fn header(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        tab_view: &TabView,
    ) -> Self {
        let toolbar_view = ToolbarView::builder().build();

        toolbar_view.add_top_bar(&Box::bar((browser_action, window_action), tab_view));

        toolbar_view
    }
}
