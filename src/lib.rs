// librdiff(rust) -- library for network deltas
// Copyright 2015, 2016, 2018 Martin Pool.

//! Compute and apply deltas from a file signature to a new file.
//!
//! This uses the same algorithm (but not the same format) as `rsync`,
//! and the same algorithm and format as `rdiff`.
//!
//! Homepage: <https://github.com/sourcefrog/librdiff-rs>.

pub mod rollsum;

/// Semver string for this library.
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
