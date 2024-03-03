//! 'Useful' synchronization extensions.
//!
//!
//! [`spsc`]: crate::sync::spsc
extern crate alloc;
pub use alloc::sync::{Arc, Weak};
pub use core::sync::atomic;

pub mod mpmc;
pub mod spsc;
