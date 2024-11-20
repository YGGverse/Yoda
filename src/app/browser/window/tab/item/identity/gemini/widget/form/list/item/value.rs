#[derive(Debug)]
pub enum Value {
    CREATE_NEW_AUTH,
    REMOVE_CURRENT_AUTH,
    PROFILE_IDENTITY_GEMINI_ID(i64),
}
