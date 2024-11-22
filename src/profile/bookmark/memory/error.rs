#[derive(Debug)]
pub enum Error {
    NotFound,
    Overwrite(i64),
}
