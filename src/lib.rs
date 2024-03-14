#![doc(html_root_url = "https://docs.rs/mvc-rs/0.1.0")]
//! mvc traits for Rust
//!

use std::error::Error;

/// trait View
pub trait View<T> {
  /// wr
  fn wr(&mut self, x: u16, y: u16, st: u16,
    bgc: u16, fgc: u16, msg: &String) -> Result<(), Box<dyn Error>>;
  /// reg
  fn reg(&mut self, c: Vec<T>) -> ();
  /// col
  fn col(&self, n: u16) -> T;
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;

  /// test a
  #[test]
  fn test_a() {
    assert_eq!(true, true);
  }
}
