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

    pub fn new(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
        tab_view: &TabView,
    ) -> Self {
        // Init components
        let bar = Rc::new(Bar::new((browser_action, window_action), profile, tab_view));

        // Return new struct
        Self {
            widget: Rc::new(Widget::new(&bar.widget.g_box)),
        }
    }
}
