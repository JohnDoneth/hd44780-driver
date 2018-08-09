#![no_std]

extern crate embedded_hal;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::OutputPin;

pub mod bus;

use bus::{DataBus, EightBitBus, FourBitBus};

pub mod entry_mode;

use entry_mode::{CursorMode, EntryMode};

pub mod display_mode;

pub use display_mode::DisplayMode;

pub struct HD44780<D: DelayUs<u16> + DelayMs<u8>, B: DataBus> {
    bus: B,
    delay: D,
    entry_mode: EntryMode,
    display_mode: DisplayMode,
}

/// Used in the direction argument for shifting the cursor and the display
pub enum Direction {
    Left,
    Right,
}

impl<
        D: DelayUs<u16> + DelayMs<u8>,
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > HD44780<D, EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>>
{
    /// Create an instance of a `HD44780` from 8 data pins, a register select
    /// pin, an enable pin and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to
    /// ensure the `HD44780` has enough time to process commands.
    /// - The eight db0..db7 pins are used to send and recieve with
    ///  the `HD44780`.
    /// - The register select pin is used to tell the `HD44780`
    /// if incoming data is a command or data.
    /// - The enable pin is used to tell the `HD44780` that there
    /// is data on the 8 data pins and that it should read them in.
    ///
    pub fn new_8bit(
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: D,
    ) -> HD44780<D, EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>> {
        let mut hd = HD44780 {
            bus: EightBitBus::from_pins(rs, en, d0, d1, d2, d3, d4, d5, d6, d7),
            delay,
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_8bit();

        return hd;
    }
}

impl<
        D: DelayUs<u16> + DelayMs<u8>,
        RS: OutputPin,
        EN: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > HD44780<D, FourBitBus<RS, EN, D4, D5, D6, D7>>
{
    /// Create an instance of a `HD44780` from 4 data pins, a register select
    /// pin, an enable pin and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to
    /// ensure the `HD44780` has enough time to process commands.
    /// - The four db0..db3 pins are used to send and recieve with
    ///  the `HD44780`.
    /// - The register select pin is used to tell the `HD44780`
    /// if incoming data is a command or data.
    /// - The enable pin is used to tell the `HD44780` that there
    /// is data on the 4 data pins and that it should read them in.
    ///
    /// This mode operates differently than 8 bit mode by using 4 less
    /// pins for data, which is nice on devices with less I/O although
    /// the I/O takes a 'bit' longer
    ///
    /// Instead of commands being sent byte by byte each command is
    /// broken up into it's upper and lower nibbles (4 bits) before
    /// being sent over the data bus
    ///
    pub fn new_4bit(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: D,
    ) -> HD44780<D, FourBitBus<RS, EN, D4, D5, D6, D7>> {
        let mut hd = HD44780 {
            bus: FourBitBus::from_pins(rs, en, d4, d5, d6, d7),
            delay,
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_4bit();

        return hd;
    }
}

impl<D, B> HD44780<D, B>
where
    D: DelayUs<u16> + DelayMs<u8>,
    B: DataBus,
{
    /// Unshifts the display and sets the cursor position to 0
    ///
    /// ```rust,ignore
    /// lcd.reset();
    /// ```
    pub fn reset(&mut self) {
        self.write_command(0b0000_0010);
    }

    /// Set if the display should be on, if the cursor should be
    /// visible, and if the cursor should blink
    ///
    /// Note: This is equivilent to calling all of the other relavent
    /// methods however this operation does it all in one go to the `HD44780`
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.display_mode = display_mode;

        let cmd_byte = self.display_mode.as_byte();

        self.write_command(cmd_byte);
    }

    /// Clear the entire display
    ///
    /// ```rust,ignore
    /// lcd.clear();
    /// ```
    pub fn clear(&mut self) {
        self.write_command(0b0000_0001);
    }

    /// If enabled, automatically scroll the display when a new
    /// character is written to the display
    ///
    /// ```rust,ignore
    /// lcd.set_autoscroll(true);
    /// ```
    pub fn set_autoscroll(&mut self, enabled: bool) {
        self.entry_mode.shift_mode = enabled.into();

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd);
    }

    /// Set if the cursor should be visible
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.display_mode.cursor_visible = visible;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd);
    }

    /// Set if the characters on the display should be visible
    pub fn set_display_visible(&mut self, visible: bool) {
        self.display_mode.display_visible = visible;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd);
    }

    /// Set if the cursor should blink
    pub fn set_cursor_blink(&mut self, blink: bool) {
        self.display_mode.cursor_blink = blink;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd);
    }

    /// Set which way the cursor will move when a new character is written
    ///
    /// ```rust,ignore
    /// // Move right (Default) when a new character is written
    /// lcd.set_cursor_mode(CursorMode::Right)
    ///
    /// // Move left when a new character is written
    /// lcd.set_cursor_mode(CursorMode::Left)
    /// ```
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.entry_mode.cursor_mode = mode;

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd);
    }

    /// Set the cursor position
    ///
    /// ```rust,ignore
    /// // Move to line 2
    /// lcd.set_cursor_pos(40)
    /// ```
    pub fn set_cursor_pos(&mut self, position: u8) {
        let lower_7_bits = 0b0111_1111 & position;

        self.write_command(0b1000_0000 | lower_7_bits);
    }

    /// Shift just the cursor to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_cursor(Direction::Left);
    /// lcd.shift_cursor(Direction::Right);
    /// ```
    pub fn shift_cursor(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_0000 | bits | bits);
    }

    /// Shift the entire display to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_display(Direction::Left);
    /// lcd.shift_display(Direction::Right);
    /// ```
    pub fn shift_display(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_1000 | bits);
    }

    /// Writes an entire string to the `HD44780`
    ///
    /// ```rust,ignore
    /// lcd.write_str("Hello, world!");
    /// ```
    pub fn write_str(&mut self, string: &str) {
        for c in string.chars() {
            self.write_char(c);
        }
    }

    /// Write a single character to the `HD44780`
    ///
    /// ```rust,ignore
    /// lcd.write_char('A');
    /// ```
    pub fn write_char(&mut self, data: char) {
        self.bus.write(data as u8, true, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    fn write_command(&mut self, cmd: u8) {
        self.bus.write(cmd, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    fn init_4bit(&mut self) {
        // Wait for the LCD to wakeup if it was off
        self.delay.delay_ms(15u8);

        // Initialize Lcd in 4-bit mode
        self.bus.write(0x33, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_ms(5u8);

        // Sets 4-bit operation and enables 5x7 mode for chars
        self.bus.write(0x32, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        self.bus.write(0x28, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Clear Display
        self.bus.write(0x0E, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Move the cursor to beginning of first line
        self.bus.write(0x01, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Set entry mode
        self.bus
            .write(self.entry_mode.as_byte(), false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        self.bus.write(0x80, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    // Follow the 8-bit setup procedure as specified in the HD44780 datasheet
    fn init_8bit(&mut self) {
        // Wait for the LCD to wakeup if it was off
        self.delay.delay_ms(15u8);

        // Initialize Lcd in 8-bit mode
        self.bus.write(0b0011_0000, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_ms(5u8);

        // Sets 8-bit operation and enables 5x7 mode for chars
        self.bus.write(0b0011_1000, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        self.bus.write(0b0000_1110, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Clear Display
        self.bus.write(0b0000_0001, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Move the cursor to beginning of first line
        self.bus.write(0b000_0111, false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Set entry mode
        self.bus
            .write(self.entry_mode.as_byte(), false, &mut self.delay);

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    // Send a byte to the HD44780 by setting the data on the bus and
    // also pulsing the enable pin
    /*fn send_byte(&mut self, data: u8) {
        // Pulse the enable pin
        self.set_bus_bits(data);
        self.pulse_enable();
    }*/

    // Pulse the enable pin telling the HD44780 that we something for it
    /*fn pulse_enable(&mut self) {
        self.en.set_high();
        self.delay.delay_ms(15u8);
        self.en.set_low();
    }*/
}
