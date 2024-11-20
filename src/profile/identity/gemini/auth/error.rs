#[derive(Debug)]
pub enum Error {
    DatabaseIndex,
    DatabaseRecordCreate(i64, String, sqlite::Error),
    DatabaseRecordDelete(i64, sqlite::Error),
    DatabaseRecordsRead(String),
    MemoryIndex(super::memory::Error),
}
