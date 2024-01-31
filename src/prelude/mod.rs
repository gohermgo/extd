// Based on prelude v1.rs, should allow me to use the feature `prelude_import`, without breaking downstream crates...
pub use std::prelude::rust_2021::*;

pub use crate::iter::{IndexableIterator, OptionIterator, ResultIterator};
