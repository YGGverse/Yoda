#[derive(Debug)]
pub enum Error {
    DatabaseIndex,
    DatabaseRecordCreate(i64, String),
    DatabaseRecordDelete(i64),
    DatabaseRecordsRead(String),
    MemoryIndex,
}
