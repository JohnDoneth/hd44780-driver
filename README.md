# hd44780-hal

[![crates.io](https://img.shields.io/crates/v/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)
[![crates.io](https://img.shields.io/crates/d/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)
[![crates.io](https://img.shields.io/crates/l/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)

Implementation of the `embedded-hal` traits for the HD44780.

![](/header.gif)


### Documentation

Crates.io - https://docs.rs/hd44780-hal

### Examples

Currently there are basic examples for **Raspberry Pi** as well as the **Adafruit Metro Express M0** as those are the devices I currently have on hand. 

Any platform that implements the [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits is supported by this library! See [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates) for a list of supported platforms.

### Getting Started

This library aims to keep it simple in that to get started all you will have to do is supply the `HD44780::new` function a bunch of pins from your platform that implement the `OutputPin` trait for [embedded-hal](https://github.com/rust-embedded/embedded-hal) as well as a struct that implements the delay traits `DelayUs<u16>`  and `DelayMs<u8>`.

```rust
// Code grabbed from the metro_m0 example
let mut lcd = HD44780::new_4bit(
    pins.d4.into_open_drain_output(&mut pins.port), // Register Select pin
    pins.d3.into_open_drain_output(&mut pins.port), // Enable pin

    pins.d9.into_open_drain_output(&mut pins.port),  // d4
    pins.d10.into_open_drain_output(&mut pins.port), // d5
    pins.d11.into_open_drain_output(&mut pins.port), // d6
    pins.d12.into_open_drain_output(&mut pins.port), // d7

    delay,
);

// Unshift display and set cursor to 0
lcd.reset(); 

// Clear existing characters
lcd.clear(); 

// Enable the display, enable cursor and blink the cursor
lcd.set_display_mode(true, true, true);

// Display the following string
lcd.write_str("Hello, world!");

// Move the cursor to the second line
lcd.set_cursor_pos(40);

// Display the following string on the second line
lcd.write_str("I'm on line 2!");
```

### Features
- 4-bit & 8-bit modes are supported

### Todo
- Busy flag support (Waiting for support from [embedded-hal](https://github.com/rust-embedded/embedded-hal) to read and write from a pin)
- Non-blocking API
- A more user-friendly API with additional features
- Custom characters

### Contributing

- Additional issues as well as pull-requests are welcome.

- If you have a platform not yet covered in this repository that is supported by [embedded-hal](https://github.com/rust-embedded/embedded-hal), a pull-request of an example would be awesome!

### License

This project is licensed under MIT license ([LICENSE](https://github.com/kunerd/clerk/blob/master/docs/CONTRIBUTING.md) or <https://opensource.org/licenses/MIT>)
