# avr-pin

A basic AVR rust pin implementation.

**Currently only supports the ATMega328P.**

## Usage

This crate has rust docs. Generate them with `$ cargo doc -r`.

**NOTE:** Due to the memory limitations of AVR microcontrollers, it is highly memory-inefficient to store I/O register addresses for each pin. For this reason, it is recommended to follow Arduino's pid approach and generating a Pin instance only for the time it's needed, dropping it immediately afterwards.

