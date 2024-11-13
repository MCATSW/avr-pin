#![no_std]

mod atmega328p;

#[cfg(feature = "atmega328p")]
pub use atmega328p::*;
