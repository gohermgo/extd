#![allow(internal_features)]
#![feature(allocator_api)] // used for RawVec, allows for specifications regarding allocator
#![feature(associated_type_defaults)]
#![feature(dropck_eyepatch)] // used for RawVec Drop impl, allows #[may_dangle]
#![feature(hint_assert_unchecked)] // used for RawVec memory-related functions, allows optimizer hints regarding memory usage etc.
#![feature(inline_const)] // used for RawVec, allows inline compile-time asserts
#![feature(negative_impls)]
#![feature(ptr_internals)] // used for RawVec's backing (Unique) pointer, especially during instantiation
#![feature(rustc_attrs)] // used for RawVec's Cap, allows platform specific capacity bounds etc.
#![feature(sized_type_properties)] // used for RawVec's ZST optimizations, ensuring unsized types get no allocation
#![feature(strict_provenance)]
#![feature(try_reserve_kind)] // used for RawVec's try-functions
#![feature(unchecked_math)] // used for RawVec's internal calculations, verified ahead of time usually
#![feature(prelude_import)]

#[prelude_import]
#[allow(unused)]
use prelude::*;

// #[prelude_import]
// #[allow(unused)]
// use std::prelude::rust_2021::*;

pub mod prelude;

pub mod collections;
pub mod raw_vec;
pub mod sync;

pub mod iter;
pub use iter::{IndexableIterator, OptionIterator, ResultIterator};
pub mod num;
