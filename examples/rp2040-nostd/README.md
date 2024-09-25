# RP2040 `no_std` Examples

To run any example, pass the example name (`src/bin/<name>.rs`):

```rs
cargo run --release --bin <name>
```

You can change the flashing method in `.cargo/config.toml`.

## Pinout for Raspberry Pi Pico

For the I2C examples, GPIO pins 18 (SDA) and 19 (SCL) were used.
If you would rather use a different configuration, make sure to update the example code accordingly.

[→ to the full pinout of this development board](https://www.waveshare.com/raspberry-pi-pico-h.htm)
