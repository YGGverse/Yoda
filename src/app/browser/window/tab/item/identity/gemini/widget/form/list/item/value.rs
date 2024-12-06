#[derive(Debug)]
pub enum Value {
    GeneratePem,
    GuestSession,
    ImportPem,
    ProfileIdentityGeminiId(i64),
}
