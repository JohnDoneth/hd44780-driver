# hd44780 Example for the Arduino Uno

Based on `avr-hal` [arduino uno examples](https://github.com/Rahix/avr-hal/tree/master/boards/arduino-uno)

## Circuit

Based on Arduino LiquidCrystal [HelloWorld example](https://www.arduino.cc/en/Tutorial/HelloWorld?from=Tutorial.LiquidCrystal)

![circuit-diagram](https://www.arduino.cc/en/uploads/Tutorial/LCD_Base_bb_Fritz.png)

## Building
```
rustup override set nightly
cargo build
```

## Flashing
```
cargo run
```
