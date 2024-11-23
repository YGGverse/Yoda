#[derive(Debug)]
pub enum Error {
    Auth(super::auth::Error),
    Certificate(Box<dyn std::error::Error>), // @TODO
    Database(sqlite::Error),
    Memory(super::memory::Error),
}
