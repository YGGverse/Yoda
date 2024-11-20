#[derive(Debug)]
pub enum Error {
    AuthInit(super::auth::Error),
    DatabaseIndex,
    DatabaseRecordCreate,
    MemoryIndex,
    Certificate(Box<dyn std::error::Error>),
}
