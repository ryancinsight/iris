//! Domain-neutral scientific visualization contracts.
//!
//! Iris owns borrowed result views, normalized color laws, fixed-size lookup
//! tables, and the render-backend seam. Domain solvers retain their physics,
//! storage providers retain arrays, and format providers retain encoding.
//!
//! The crate is `no_std` compatible with an allocator when default features
//! are disabled.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod color;
pub mod error;
pub mod render;
pub mod view;

pub use error::{IrisError, IrisResult};
