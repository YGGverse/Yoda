use gtk::{Separator, prelude::WidgetExt};

pub fn horizontal(text_view: &gtk::TextView) -> Separator {
    const MARGIN: i32 = 8;
    let separator = Separator::builder()
        .margin_bottom(MARGIN)
        .margin_top(MARGIN)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    gtk::glib::idle_add_local({
        let text_view = text_view.clone();
        let separator = separator.clone();
        move || {
            separator.set_width_request(text_view.width() - 18);
            gtk::glib::ControlFlow::Break
        }
    });
    separator
}
