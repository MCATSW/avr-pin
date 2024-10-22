//! I/O register addresses

pub const PINB: *mut u8 = 0x0023 as *mut u8;
pub const DDRB: *mut u8 = 0x0024 as *mut u8;
pub const PORTB: *mut u8 = 0x0025 as *mut u8;
pub const PINC: *mut u8 = 0x0026 as *mut u8;
pub const DDRC: *mut u8 = 0x0027 as *mut u8;
pub const PORTC: *mut u8 = 0x0028 as *mut u8;
pub const PIND: *mut u8 = 0x0029 as *mut u8;
pub const DDRD: *mut u8 = 0x002A as *mut u8;
pub const PORTD: *mut u8 = 0x002B as *mut u8;
