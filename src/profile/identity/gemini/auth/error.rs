#[derive(Debug)]
pub enum Error {
    DatabaseIndex(sqlite::Error),
    DatabaseRecordCreate(i64, String, sqlite::Error),
    DatabaseRecordDelete(i64, sqlite::Error),
    DatabaseRecordsRead(String, sqlite::Error),
    MemoryIndex(super::memory::Error),
}
