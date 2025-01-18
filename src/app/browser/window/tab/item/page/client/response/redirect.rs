use crate::app::browser::window::tab::item::page::client::Request;

pub enum Redirect {
    Foreground(Request),
    Background(Request),
}
