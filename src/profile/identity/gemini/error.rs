#[derive(Debug)]
pub enum Error {
    AuthInit,
    DatabaseIndex,
    DatabaseRecordCreate,
    MemoryIndex,
    Certificate(Box<dyn std::error::Error>),
}
