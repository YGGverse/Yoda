#[derive(Debug)]
pub enum Error {
    DatabaseActive(sqlite::Error),
    DatabaseAdd(sqlite::Error),
    GeminiInit(super::gemini::Error),
}
