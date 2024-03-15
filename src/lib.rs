#![doc(html_root_url = "https://docs.rs/mvc-rs/0.3.2")]
//! mvc traits for Rust
//!

use std::error::Error;

/// trait TPacket
pub trait TPacket {
  /// as_str
  fn as_str(&self) -> &str;
  /// to_vec
  fn to_vec(&self) -> Vec<u16>;
  /// flat
  fn flat(&self) -> Vec<u8>;
}

/// trait TView
pub trait TView<T> {
  /// wr
  fn wr(&mut self, p: impl TPacket) -> Result<(), Box<dyn Error>>;
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
