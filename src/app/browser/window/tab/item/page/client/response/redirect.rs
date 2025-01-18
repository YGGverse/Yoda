use super::super::Request;

pub enum Redirect {
    Foreground(Request),
    Background(Request),
}
