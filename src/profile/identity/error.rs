#[derive(Debug)]
pub enum Error {
    Database,
    GeminiInit(super::gemini::Error),
}
