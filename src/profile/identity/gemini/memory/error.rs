#[derive(Debug)]
pub enum Error {
    Clear,
    NotFound(i64),
    Overwrite(i64),
    Unexpected,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => format!("Could not cleanup memory index"),
            Self::NotFound(key) => {
                format!("Record `{key}` not found in memory index")
            }
            Self::Overwrite(key) => {
                format!("Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => format!("Unexpected error"),
        }
    }
}
