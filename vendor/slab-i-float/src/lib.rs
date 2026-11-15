#![cfg_attr(not(target_os = "windows"), no_std)]
#![cfg_attr(feature = "core", doc = include_str!("../README.md"))]
#![cfg_attr(
    not(feature = "core"),
    doc = "Enable the `core` feature (enabled by default) to use the numeric and geometry API."
)]
extern crate alloc;

#[cfg(feature = "core")]
pub mod adapter;
#[cfg(feature = "core")]
pub mod float;
#[cfg(feature = "core")]
pub mod int;
#[cfg(feature = "core")]
pub mod triangle;

pub mod integration;


// Slab Windows backend: same floating-point operations through the installed
// Rust/Windows execution environment; no separately bundled libm implementation.
#[cfg(not(target_os = "windows"))]
pub(crate) use libm as math;
#[cfg(target_os = "windows")]
pub(crate) mod math;
