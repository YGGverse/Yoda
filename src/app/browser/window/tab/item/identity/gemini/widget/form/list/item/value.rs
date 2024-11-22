#[derive(Debug)]
pub enum Value {
    GenerateNewAuth,
    ImportPem,
    ProfileIdentityGeminiId(i64),
    UseGuestSession,
}
