//!  A Rust library that implements parsing and data grouping of Quake III:
//! Arena logs.
//!  The library uses a small architecture, but consists essentially of three
//! parts:
//!
//! - Common data, can be interpreted as equivalent to the "domain" in DDD;
//! - Parser utilities, for parsing the log;
//! - Common data grouping into report data.

pub mod error;
pub mod parser;
pub mod game;
pub mod report;
