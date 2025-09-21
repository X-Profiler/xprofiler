#![deny(clippy::all)]

use napi_derive::napi;

// Core modules
pub mod config;
pub mod environment;
pub mod logger;
pub mod monitoring;
pub mod utils;
pub mod bindings;

// Re-export main functionality
pub use config::*;
pub use environment::*;
pub use logger::*;
pub use monitoring::*;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
