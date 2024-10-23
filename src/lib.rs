#![no_std]

pub mod addresses;
pub mod pid;

/// A structure containing I/O register
/// pointers and a mask for a given `Pin`.
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
    /// Creates a `Pin` based on an Arduino pin ID.
    ///
    /// If the pid is valid, `Some` containing a `Pin` is returned.
    /// Otherwise, `None` is returned.
    pub fn from_pid(pid: u8) -> Option<Self> {
        if pid < 8 {
            Some(Self {
                ddr: addresses::DDRD,
                port: addresses::PORTD,
                pin: addresses::PIND,
                mask: 1 << pid,
            })
        } else if pid < 14 {
            Some(Self {
                ddr: addresses::DDRB,
                port: addresses::PORTB,
                pin: addresses::PINB,
                mask: 1 << (pid - 8),
            })
        } else if pid < 20 {
            Some(Self {
                ddr: addresses::DDRC,
                port: addresses::PORTC,
                pin: addresses::PINC,
                mask: 1 << (pid - 14),
            })
        } else {
            None
        }
    }
    /// Sets the `Pin`'s DDR value
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn set_ddr(&self, dd: DD) {
        self.ddr.write_volatile(match dd {
            DD::Input => self.ddr.read_volatile() & !self.mask,
            DD::Output => self.ddr.read_volatile() | self.mask,
        });
    }
    /// Modifies the `Pin`'s PORT value
    ///
    /// When the output mode is set, this sets the pin state.
    /// When the input mode is set, this enables/disables the pullup resistor.
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn write(&self, state: bool) {
        self.port.write_volatile(match state {
            true => self.port.read_volatile() | self.mask,
            false => self.port.read_volatile() & !self.mask,
        });
    }
    /// Fetches the `Pin`'s PIN value
    ///
    /// Considering the input mode is set, this returns the pin state.
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn read(&self) -> bool {
        (self.pin.read_volatile() & self.mask) > 0
    }
    /// Sets the `Pin` to input mode with the internal pullup resistor disabled.
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn set_input(&self) {
        self.set_ddr(DD::Input);
        self.write(false);
    }
    /// Sets the `Pin` to input mode with the internal pullup resistor enabled.
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn set_input_pullup(&self) {
        self.set_ddr(DD::Input);
        self.write(true);
    }
    /// Sets the `Pin` to output mode
    ///
    /// # Safety
    ///
    /// Since the `Pin` struct can be constructed by the user,
    /// there is no guarantee that the I/O register addresses
    /// are valid. Please ensure validity of self, ideally by
    /// avoiding manual generation of `Pin`.
    pub unsafe fn set_output(&self) {
        self.set_ddr(DD::Output);
    }
}
