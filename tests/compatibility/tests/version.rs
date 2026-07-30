#[test]
fn package_version_is_available() {
    assert!(!rubase::version().is_empty());
}
