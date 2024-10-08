#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

mod as_any;
mod cursor;
mod decode;
mod encode;
mod error;
mod into_owned;
#[cfg(feature = "alloc")]
mod write_buf;

// Flat API hierarchy of common traits and types

pub use self::as_any::*;
pub use self::cursor::*;
pub use self::decode::*;
pub use self::encode::*;
pub use self::error::*;
pub use self::into_owned::*;
#[cfg(feature = "alloc")]
pub use self::write_buf::*;
