#[cfg(test)]
#[test]
fn strip_prefix() {
    assert_eq!(super::strip_prefix("source:gemini".into()), "gemini");
    assert_eq!(super::strip_prefix("download:gemini".into()), "gemini");
}
