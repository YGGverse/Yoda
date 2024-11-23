#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Gemini(super::gemini::Error),
}
