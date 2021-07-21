// Turn on `no_std`
#![cfg_attr(not(feature = "std"), no_std)]

/// Defines system function and component call protocol.
pub mod abi;
/// A module that handles buffer encoding and decoding.
pub mod buffer;
/// Scrypto higher level abstraction.
pub mod constructs;
/// A module that facilitates system calls.
pub mod kernel;
/// A library of common routines.
pub mod library;
#[doc(hidden)]
pub mod macros;
/// Scrypto primitive types.
pub mod types;
/// Utility functions, such as hashing and hex decoding.
pub mod utils;