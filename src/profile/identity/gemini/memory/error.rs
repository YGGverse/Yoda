#[derive(Debug)]
pub enum Error {
    Clear,
    NotFound(i64),
    Overwrite(String),
}
