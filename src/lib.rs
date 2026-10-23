//! Byte-oriented PHP 8.3 syntax parsing. No input is executed.
#![forbid(unsafe_code)]

pub mod contracts;
mod syntax;

pub use syntax::parse;
