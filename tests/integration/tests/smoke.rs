#[test]
fn public_library_is_linkable() {
    assert_eq!(rubase::version(), env!("CARGO_PKG_VERSION"));
}
