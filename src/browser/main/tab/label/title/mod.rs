use gtk::Label;

pub fn new() -> Label {
    Label::builder()
        .label("New page")
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .width_chars(16)
        .single_line_mode(true)
        .build()
}
