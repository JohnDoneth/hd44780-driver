# hd44780-driver

[![crates.io](https://img.shields.io/crates/v/hd44780-driver.svg)](https://crates.io/crates/hd44780-driver)
[![crates.io](https://img.shields.io/crates/d/hd44780-driver.svg)](https://crates.io/crates/hd44780-driver)
[![crates.io](https://img.shields.io/crates/l/hd44780-driver.svg)](https://crates.io/crates/hd44780-driver)
[![travis-ci.org](https://travis-ci.org/JohnDoneth/hd44780-driver.svg?branch=master)](https://travis-ci.org/JohnDoneth/hd44780-driver)
[![API](https://docs.rs/hd44780-driver/badge.svg)](https://docs.rs/hd44780-driver)

Implementation of the `embedded-hal` traits for the HD44780.

![](/header.gif)

### Examples

Examples for several different boards can be found [here](/examples)

Any platform that implements the [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits is supported by this library! See [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates) for a list of supported platforms.

### Getting Started

This library aims to keep it simple in that to get started all you will have to do is supply the `HD44780::new` function a bunch of pins from your platform that implement the `OutputPin` trait for [embedded-hal](https://github.com/rust-embedded/embedded-hal) as well as a struct that implements the delay traits `DelayUs<u16>`  and `DelayMs<u8>`.

```rust
// Pseudo-code: check the HAL crate for your specific device for exact code to get pins / delay
// It is recommended to use push/pull output pins, but if your specific LCD device has pull-up resistors
// an open/drain output pin should work too

let mut delay = Delay::new();

let mut lcd = HD44780::new_4bit(
    d4.into_push_pull_output(&mut port), // Register Select pin
    d3.into_push_pull_output(&mut port), // Enable pin

    d9.into_push_pull_output(&mut port),  // d4
    d10.into_push_pull_output(&mut port), // d5
    d11.into_push_pull_output(&mut port), // d6
    d12.into_push_pull_output(&mut port), // d7
    &mut delay,
);

// Unshift display and set cursor to 0
lcd.reset(&mut delay); 

// Clear existing characters
lcd.clear(&mut delay); 

// Display the following string
lcd.write_str("Hello, world!", &mut delay);

// Move the cursor to the second line
lcd.set_cursor_pos(40, &mut delay);

// Display the following string on the second line
lcd.write_str("I'm on line 2!", &mut delay);
```

### Features
- 4-bit & 8-bit modes are supported
- Support for i2c backpacks

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
