#[derive(Debug)]
pub enum Error {
    Clear,
    Overwrite(i64),
}
