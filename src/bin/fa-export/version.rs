//! versioning stuff

use clap::{crate_authors, crate_description, crate_version};

/// Binary name, using a different binary name
pub(crate) const NAME: &str = env!("CARGO_BIN_NAME");
/// Binary version
pub(crate) const VERSION: &str = crate_version!();
/// Authors
pub(crate) const AUTHORS: &str = crate_authors!();

use flightaware_rs as api;

/// Display our CLI version banner
///
/// Example:
/// ```
/// # use version::version;
///
/// println!(version());
/// ```
///
#[inline]
pub fn version() -> String {
    return format!(
        "{}/{} API/{} by {}\n{}",
        NAME,
        VERSION,
        api::API_VERSION,
        AUTHORS,
        crate_description!()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(version().contains(NAME));
        assert!(version().contains(VERSION));
        assert!(version().contains(AUTHORS))
    }
}
