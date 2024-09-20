use gtk::Label;

pub fn new() -> Label {
    let title = Label::builder()
        .css_classes(["title"])
        .single_line_mode(true)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    update(&title, "Welcome");

    return title;
}

pub fn update(title: &Label, text: &str) {
    let default_text = "Yoda"; // @TODO

    if text.is_empty() {
        title.set_text(default_text);
    } else {
        title.set_text(&format!("{} - {}", text, default_text));
    }
}
