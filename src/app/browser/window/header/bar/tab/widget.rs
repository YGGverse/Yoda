use adw::{TabBar, TabView};
use gtk::prelude::IsA;
use std::rc::Rc;

pub struct Widget {
    gobject: TabBar,
}

impl Widget {
    // Construct
    pub fn new_rc(view: &TabView, start_action_widget: &impl IsA<gtk::Widget>) -> Rc<Self> {
        Rc::new(Self {
            gobject: TabBar::builder()
                .autohide(false)
                .expand_tabs(false)
                .end_action_widget(start_action_widget) // @TODO find solution to append after tabs
                .view(view)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        &self.gobject
    }
}
