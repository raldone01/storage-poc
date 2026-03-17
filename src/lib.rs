#![cfg_attr(not(test), no_std)]
//  Language Features
#![feature(coerce_unsized)]
#![feature(ptr_metadata)]
#![feature(unsize)]
//  Library Features
#![feature(allocator_api)]
#![feature(layout_for_ptr)]
#![feature(slice_ptr_get)]
//  Lints
#![deny(missing_docs)]

//! TODO

pub mod allocator;
pub mod alternative;
pub mod collections;
pub mod fallback;
pub mod inline;
pub mod small;
pub mod traits;

mod utils;
