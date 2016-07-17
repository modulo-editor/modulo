//! This crate stores the modulo core logic. This is the backbone of the modulo project.

#[macro_use]
extern crate log;
extern crate modulo_traits;

#[cfg(test)]
pub mod tests;
pub mod core;
pub mod file;
