# ESP32 `no_std` Examples

To run any example, pass the example name (`src/bin/<name>.rs`) to espflash:

```rs
cargo espflash flash --monitor --release --bin <name>
```

For the 8-pin example, the used pins overlap with those used for flashing. You might need to disconnect 5V from the LCD display before flashing works. This was done for convenience, such that all 8 pins are physically in one row.
