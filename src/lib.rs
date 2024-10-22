#![no_std]

use core::panic;

pub mod addresses;

/// A structure containing I/O register
/// pointers and a mask for a given Pin.
#[repr(packed)]
pub struct Pin {
    pub ddr: *mut u8,
    pub port: *mut u8,
    pub pin: *mut u8,
    pub mask: u8,
}

/// An enum representing pin data direction.
#[repr(u8)]
pub enum DD {
    Input,
    Output,
}

impl Pin {
    /// Creates a Pin based on an Arduino pin ID.
    ///
    /// # Panics
    ///
    /// Panics when an invalid pid is supplied.
    pub fn from_pid(pid: u8) -> Self {
        if pid < 8 {
            Self {
                ddr: addresses::DDRD,
                port: addresses::PORTD,
                pin: addresses::PIND,
                mask: 1 << pid,
            }
        } else if pid < 14 {
            Self {
                ddr: addresses::DDRB,
                port: addresses::PORTB,
                pin: addresses::PINB,
                mask: 1 << (pid - 8),
            }
        } else if pid < 20 {
            Self {
                ddr: addresses::DDRC,
                port: addresses::PORTC,
                pin: addresses::PINC,
                mask: 1 << (pid - 14),
            }
        } else {
            panic!();
        }
    }
    /// Sets the Pin's DDR value
    pub unsafe fn set_ddr(&self, dd: DD) {
        self.ddr.write_volatile(match dd {
            DD::Input => self.ddr.read_volatile() & !self.mask,
            DD::Output => self.ddr.read_volatile() | self.mask,
        });
    }
    /// Modifies the pin's PORT value
    ///
    /// When the output mode is set, this sets the pin state.
    /// When the input mode is set, this enables/disables the pullup resistor.
    pub unsafe fn write(&self, state: bool) {
        self.port.write_volatile(match state {
            true => self.port.read_volatile() | self.mask,
            false => self.port.read_volatile() & !self.mask,
        });
    }
    /// Fetches the pin's PIN value
    ///
    /// Considering the input mode is set, this returns the pin state.
    pub unsafe fn read(&self) -> bool {
        (self.pin.read_volatile() & self.mask) > 0
    }
    /// Sets the pin to input mode with the internal pullup resistor disabled.
    pub unsafe fn set_input(&self) {
        self.set_ddr(DD::Input);
        self.write(false);
    }
    /// Sets the pin to input mode with the internal pullup resistor enabled.
    pub unsafe fn set_input_pullup(&self) {
        self.set_ddr(DD::Input);
        self.write(true);
    }
    /// Sets the pin to output mode
    pub unsafe fn set_output(&self) {
        self.set_ddr(DD::Output);
    }
}
