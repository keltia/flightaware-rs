//! API bindings for [Flightaware] API
//!
//! [Flightaware]: https://flightaware.com/
//!
//!

use clap::{crate_authors, crate_description, crate_version, StructOpt};

// Our modules
//

/// API version
pub const API_VERSION: &str = crate_version!();
/// Authors
pub(crate) const AUTHORS: &str = crate_authors!();

/// Display our version banner complete with authors & description
///
/// Example:
/// ```
/// # use flightaware_rs::description;
///
/// println!("{}", description());
/// ```
///
#[inline]
pub fn description() -> String {
    format!(
        "API/{} by {}\n{}",
        API_VERSION,
        AUTHORS,
        crate_description!()
    )
}

/// Display our minimal version banner
///
/// Example:
/// ```
/// # use flightaware_rs::version;
///
/// println!("{}", version());
/// ```
///
#[inline]
pub fn version() -> String {
    format!("API/{}", API_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_version() {
        assert_eq!(version(), format!("API/{}", API_VERSION));
    }

    #[test]
    fn test_api_description() {
        assert!(description().contains("API/"));
        assert!(description().contains(API_VERSION));
        assert!(description().contains(AUTHORS))
    }
}
