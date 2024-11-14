#[derive(Debug)]
pub enum Error {
    DatabaseAdd,
    DatabaseDelete,
    MemoryAdd,
    MemoryDelete,
    MemoryNotFound,
}
