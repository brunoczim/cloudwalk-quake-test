//! A module that exists for future-proof definitions of items used for error
//! handling.
//!
//! Right now, it sounds "silly", but it would be easier to refactor error
//! types/items.

/// Global error of this library.
pub type Error = anyhow::Error;

/// Global result type of this library (with flexible error type).
pub type Result<T, E = Error> = std::result::Result<T, E>;
