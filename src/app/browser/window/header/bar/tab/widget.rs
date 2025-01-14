use adw::{TabBar, TabView};
use gtk::prelude::IsA;

pub struct Widget {
    pub tab_bar: TabBar,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(view: &TabView, start_action_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            tab_bar: TabBar::builder()
                .autohide(false)
                .expand_tabs(false)
                .end_action_widget(start_action_widget) // @TODO find solution to append after tabs
                .view(view)
                .build(),
        }
    }
}
