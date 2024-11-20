#[derive(Debug)]
pub enum Value {
    GENERATE_NEW_AUTH,
    USE_GUEST_SESSION,
    PROFILE_IDENTITY_GEMINI_ID(i64),
}
