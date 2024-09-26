# ATMEGA328 `no_std` Examples

To run any example, pass the example name (`src/bin/<name>.rs`):

```sh
cargo run --release --bin <name>
```

If you want to use a flashing method other than USBASP, specify it like this and it will be passed to `avrdude` as the `-c` option:

```sh
cargo run --release --bin <name> -- <flasher>
```

You can change the default flashing method in `flash.sh`. If you're using an Arduino other than Nano, the flashing script needs to be updated.

## Pinout for Arduino Nano

For the I2C examples, GPIO pins A4 (SDA) and A5 (SCL) were used.
If you would rather use a different configuration, make sure to update the example code accordingly.

[→ to the full pinout of this development board](https://docs.arduino.cc/resources/pinouts/A000005-full-pinout.pdf)
