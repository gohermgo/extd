//! Single-producer, single-consumer primitives.

use crate::sync::mpmc;
use std::{
    error, fmt,
    time::{Duration, Instant},
};

pub struct Receiver<T> {
    inner: mpmc::Receiver<T>,
}

// The receiver port can be sent from place to place, so long as it
// is not used to receive non-sendable things.
unsafe impl<T: Send> Send for Receiver<T> {}

impl<T> !Sync for Receiver<T> {}
