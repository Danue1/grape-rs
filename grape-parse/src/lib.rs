#![warn(clippy::all)]

#[cfg(test)]
mod tests;

mod macros;
pub mod parse;

pub use parse::*;
