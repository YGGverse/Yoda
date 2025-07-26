const TRUE: &str = "1";
const FALSE: &str = "0";

#[derive(Eq, Hash, PartialEq, Default)]
pub enum Bool {
    True,
    #[default]
    False,
}

impl Bool {
    pub fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }

    pub fn from_db_value(key: Option<&str>) -> Self {
        if key.is_some_and(|k| k == TRUE) {
            Self::True
        } else {
            Self::False
        }
    }

    pub fn into_db_value(self) -> String {
        match self {
            Self::True => TRUE,
            Self::False => FALSE,
        }
        .to_string()
    }

    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }
}
