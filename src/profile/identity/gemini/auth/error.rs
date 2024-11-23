#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Memory(super::memory::Error),
}
