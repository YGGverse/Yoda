mod bar;
mod widget;

use bar::Bar;
use widget::Widget;

use super::{Action as WindowAction, BrowserAction, Profile};
use adw::TabView;
use std::rc::Rc;

pub struct Header {
    pub widget: Rc<Widget>,
}

impl Header {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
        tab_view: &TabView,
    ) -> Self {
        // Init components
        let bar = Rc::new(Bar::build(
            (browser_action, window_action),
            profile,
            tab_view,
        ));

        // Return new struct
        Self {
            widget: Rc::new(Widget::build(&bar.widget.g_box)),
        }
    }
}
