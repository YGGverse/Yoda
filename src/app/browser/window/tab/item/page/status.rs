/// `Page` status
/// * not same as the Gemini status!
#[derive(Debug, Clone)]
pub enum Status {
    Complete,
    Connected,
    Connecting,
    Failure,
    Input,
    New,
    ProxyNegotiated,
    ProxyNegotiating,
    Redirect,
    Reload,
    Resolved,
    Resolving,
    SessionRestore,
    SessionRestored,
    Success,
    TlsHandshaked,
    TlsHandshaking,
}
