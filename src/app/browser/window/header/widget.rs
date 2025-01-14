use adw::ToolbarView;
use gtk::prelude::IsA;

pub struct Widget {
    pub toolbar_view: ToolbarView,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(top_bar: &impl IsA<gtk::Widget>) -> Self {
        let toolbar_view = ToolbarView::builder().build();

        toolbar_view.add_top_bar(top_bar);

        Self { toolbar_view }
    }
}
