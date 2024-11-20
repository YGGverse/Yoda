#[derive(Debug)]
pub enum Error {
    Database,
    Gemini(super::gemini::Error),
}
