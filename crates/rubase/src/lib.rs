//! Public library interface for `RuBase`.

/// Returns the package version of the linked `RuBase` library.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_workspace_version() {
        assert_eq!(super::version(), env!("CARGO_PKG_VERSION"));
    }
}
