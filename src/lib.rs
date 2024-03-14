#![doc(html_root_url = "https://docs.rs/mvc-rs/0.2.0")]
//! mvc traits for Rust
//!

use std::error::Error;

/// trait View
pub trait View<P, T> {
  /// wr
  fn wr(&mut self, p: P) -> Result<(), Box<dyn Error>>;
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
