#[derive(Debug)]
pub enum Value {
    GENERATE_NEW_AUTH,
    IMPORT_PEM,
    PROFILE_IDENTITY_GEMINI_ID(i64),
    USE_GUEST_SESSION,
}
