#[derive(Debug)]
pub enum Value {
    GeneratePem,
    GuestSession,
    ImportPem,
    ProfileIdentityId(i64),
}
