#[derive(Debug)]
pub enum Error {
    AuthInit(super::auth::Error),
    DatabaseIndex(sqlite::Error),
    DatabaseRecordCreate(sqlite::Error),
    MemoryIndex(i64),
    Certificate(Box<dyn std::error::Error>),
}
