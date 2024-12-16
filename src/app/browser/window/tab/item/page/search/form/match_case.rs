use gtk::{gdk::Cursor, CheckButton};

pub fn new() -> CheckButton {
    CheckButton::builder()
        .cursor(&Cursor::from_name("default", None).unwrap())
        .label("Match case")
        .build()
}
