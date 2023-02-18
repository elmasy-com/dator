//! Validate different data types.
//!
//! This library is realy simple: give a `&str` as an argument and return a `bool`.
//!
//! # Example
//! ```
//! if dator::domain("example.com") {
//!     // valid domain
//! }
//! ```

mod domain;
mod ip;
mod mail;
pub use self::domain::domain;
pub use self::ip::{ip, ipv4, ipv6};
pub use self::mail::email;
