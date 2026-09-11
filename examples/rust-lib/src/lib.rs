/// Returns the library version string.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
