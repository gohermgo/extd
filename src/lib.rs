#![allow(internal_features)]
#![feature(associated_type_defaults)]
#![feature(prelude_import)]

#[prelude_import]
#[allow(unused)]
use prelude::*;

// #[prelude_import]
// #[allow(unused)]
// use std::prelude::rust_2021::*;

pub mod prelude;

pub mod iter;
pub use iter::{IndexableIterator, OptionIterator, ResultIterator};
