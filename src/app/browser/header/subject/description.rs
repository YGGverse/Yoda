use gtk::prelude::WidgetExt;
use gtk::Label;

pub fn new() -> Label {
    let description = Label::builder()
        .css_classes(["subtitle"])
        .single_line_mode(true)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    update(
        &description,
        "", // @TODO
    );

    description
}

pub fn update(description: &Label, text: &str) {
    description.set_text(text);

    if text.is_empty() {
        description.hide();
    } else {
        description.show();
    }
}
