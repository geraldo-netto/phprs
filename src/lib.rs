//! PHP 8.x syntax target; current grammar baseline is PHP 8.5.0. No input executes.
#![forbid(unsafe_code)]

pub mod contracts;
mod syntax;

pub use syntax::PhpParser;
