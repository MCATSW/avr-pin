//! Shorthand functions for pid-generated `Pin` instances.

use crate::{Pin, DD};

/// Sets the `Pin`'s DDR value
///
/// Returns `false` if the pid was invalid.
/// Otherwise, returns `true`.
pub fn set_ddr(pid: u8, dd: DD) -> bool {
    match Pin::from_pid(pid) {
        Some(pin) => unsafe {
            pin.set_ddr(dd);
        },
        None => return false,
    }
    true
}

/// Modifies the `Pin`'s PORT value
///
/// When the output mode is set, this sets the pin state.
/// When the input mode is set, this enables/disables the pullup resistor.
///
/// Returns `false` if the pid was invalid.
/// Otherwise, returns `true`.
pub fn write(pid: u8, state: bool) -> bool {
    match Pin::from_pid(pid) {
        Some(pin) => unsafe {
            pin.write(state);
        },
        None => return false,
    }
    true
}

/// Fetches the `Pin`'s PIN value
///
/// Returns a `None` if the pid was invalid.
/// Otherwise, returns a `Some` containing the pin state.
pub fn read(pid: u8) -> Option<bool> {
    unsafe { Some(Pin::from_pid(pid)?.read()) }
}

/// Sets the `Pin` to input mode with the internal pullup resistor disabled.
///
/// Returns `false` if the pid was invalid.
/// Otherwise, returns `true`.
pub fn set_input(pid: u8) -> bool {
    match Pin::from_pid(pid) {
        Some(pin) => unsafe {
            pin.set_input();
        },
        None => return false,
    }
    true
}

/// Sets the `Pin` to input mode with the internal pullup resistor enabled.
///
/// Returns `false` if the pid was invalid.
/// Otherwise, returns `true`.
pub fn set_input_pullup(pid: u8) -> bool {
    match Pin::from_pid(pid) {
        Some(pin) => unsafe {
            pin.set_input_pullup();
        },
        None => return false,
    }
    true
}

/// Sets the `Pin` to output mode
///
/// Returns `false` if the pid was invalid.
/// Otherwise, returns `true`.
pub fn set_output(pid: u8) -> bool {
    match Pin::from_pid(pid) {
        Some(pin) => unsafe {
            pin.set_output();
        },
        None => return false,
    }
    true
}

