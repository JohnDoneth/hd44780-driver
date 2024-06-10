# hd44780-driver

[![crates.io](https://img.shields.io/crates/v/hd44780-driver.svg)](https://crates.io/crates/hd44780-driver)
[![crates.io](https://img.shields.io/crates/l/hd44780-driver.svg)](https://crates.io/crates/hd44780-driver)
[![travis-ci.org](https://travis-ci.org/JohnDoneth/hd44780-driver.svg?branch=master)](https://travis-ci.org/JohnDoneth/hd44780-driver)
[![Rust](https://github.com/JohnDoneth/hd44780-driver/actions/workflows/rust.yml/badge.svg)](https://github.com/JohnDoneth/hd44780-driver/actions/workflows/rust.yml)
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

### Async API

The async API is similar to the sync API. The the major differences are that:
- The async API requires the `async` feature to use.
- The async API requires the nightly compiler because of use of unstable features.
- The async API uses `embedded-hal-async` rather than `embedded-hal` traits.

Embassy provides some implementations of these traits for some MCUs, and provides
an executor that can execute futures. However, projects implementing `embedded-hal-async` traits,
including this project, can run on any executor with any driver, provided such
executor and driver also implement `embedded-async-traits`.

```rust
use hd44780_driver::non_blocking::HD44780;

let mut delay = embassy::time::Delay::new();
pin_mut!(delay);

let mut display = HD44780::new_4bit(
    rs,
    en,
    d4,
    d5,
    d6,
    d7,
    delay.as_mut(),
)
.await
.unwrap();

display.clear(delay.as_mut()).await;
display.write_str(msg, delay.as_mut()).await;
```

### Features
- 4-bit & 8-bit modes are supported
- Support for I2C backpacks based on PCF8574 and MCP23008 port expanders
- Non-blocking API

### Todo
- Busy flag support
- A more user-friendly API with additional features
- Custom characters

### Contributing

- Additional issues as well as pull-requests are welcome.

- If you have a platform not yet covered in this repository that is supported by [embedded-hal](https://github.com/rust-embedded/embedded-hal), a pull-request of an example would be awesome!

### License

This project is licensed under MIT license ([LICENSE](https://github.com/kunerd/clerk/blob/master/docs/CONTRIBUTING.md) or <https://opensource.org/licenses/MIT>)
