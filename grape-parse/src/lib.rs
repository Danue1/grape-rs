#![warn(clippy::all)]

#[cfg(test)]
mod tests;

pub mod error;
mod macros;
pub mod parse;

pub use error::*;
pub use parse::*;
