use gtk::CheckButton;

pub fn new() -> CheckButton {
    CheckButton::builder().label("Match case").build()
}
