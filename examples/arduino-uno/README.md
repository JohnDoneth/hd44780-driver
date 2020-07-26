```
hd44780 Example for the Arduino Uno
```

Building
```
rustup override set nightly-2020-07-24-x86_64-unknown-linux-gnu

cargo build -Z build-std=core --target avr-atmega328p.json --release
```