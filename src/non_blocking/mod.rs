//use core::fmt::Result;
//use core::fmt::Write;

use embassy_traits::delay::Delay;
use embedded_hal::digital::v2::OutputPin;

pub mod bus;
use bus::{DataBus, EightBitBus, FourBitBus};

pub use crate::error;
use error::Result;

pub use crate::entry_mode;

use entry_mode::{CursorMode, EntryMode};

pub use crate::display_mode;

pub use display_mode::DisplayMode;

pub struct HD44780<B: DataBus> {
    bus: B,
    entry_mode: EntryMode,
    display_mode: DisplayMode,
}

pub use crate::Cursor;
pub use crate::CursorBlink;
pub use crate::Direction;
pub use crate::Display;

impl<
        RS: OutputPin + 'static,
        EN: OutputPin + 'static,
        D0: OutputPin + 'static,
        D1: OutputPin + 'static,
        D2: OutputPin + 'static,
        D3: OutputPin + 'static,
        D4: OutputPin + 'static,
        D5: OutputPin + 'static,
        D6: OutputPin + 'static,
        D7: OutputPin + 'static,
    > HD44780<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>>
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
    pub async fn new_8bit<'a, D: Delay>(
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
        delay: &'a mut D,
    ) -> Result<HD44780<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>>> {
        let mut hd = HD44780 {
            bus: EightBitBus::from_pins(rs, en, d0, d1, d2, d3, d4, d5, d6, d7),
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_8bit(delay).await?;

        return Ok(hd);
    }
}

impl<
        RS: OutputPin + 'static,
        EN: OutputPin + 'static,
        D4: OutputPin + 'static,
        D5: OutputPin + 'static,
        D6: OutputPin + 'static,
        D7: OutputPin + 'static,
    > HD44780<FourBitBus<RS, EN, D4, D5, D6, D7>>
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
    pub async fn new_4bit<'a, D: Delay>(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: &'a mut D,
    ) -> Result<HD44780<FourBitBus<RS, EN, D4, D5, D6, D7>>> {
        let mut hd = HD44780 {
            bus: FourBitBus::from_pins(rs, en, d4, d5, d6, d7),
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_4bit(delay).await?;

        return Ok(hd);
    }
}

impl<B> HD44780<B>
where
    B: DataBus,
{
    /// Unshifts the display and sets the cursor position to 0
    ///
    /// ```rust,ignore
    /// lcd.reset();
    /// ```
    pub async fn reset<'a, D: Delay>(&mut self, delay: &'a mut D) -> Result<()> {
        self.write_command(0b0000_0010, delay).await?;

        Ok(())
    }

    /// Set if the display should be on, if the cursor should be
    /// visible, and if the cursor should blink
    ///
    /// Note: This is equivilent to calling all of the other relavent
    /// methods however this operation does it all in one go to the `HD44780`
    pub async fn set_display_mode<'a, D: Delay>(
        &mut self,
        display_mode: DisplayMode,
        delay: &'a mut D,
    ) -> Result<()> {
        self.display_mode = display_mode;

        let cmd_byte = self.display_mode.as_byte();

        self.write_command(cmd_byte, delay).await?;

        Ok(())
    }

    /// Clear the entire display
    ///
    /// ```rust,ignore
    /// lcd.clear();
    /// ```
    pub async fn clear<'a, D: Delay>(&mut self, delay: &'a mut D) -> Result<()> {
        self.write_command(0b0000_0001, delay).await?;

        Ok(())
    }

    /// If enabled, automatically scroll the display when a new
    /// character is written to the display
    ///
    /// ```rust,ignore
    /// lcd.set_autoscroll(true);
    /// ```
    pub async fn set_autoscroll<'a, D: Delay>(
        &mut self,
        enabled: bool,
        delay: &'a mut D,
    ) -> Result<()> {
        self.entry_mode.shift_mode = enabled.into();

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd, delay).await?;

        Ok(())
    }

    /// Set if the cursor should be visible
    pub async fn set_cursor_visibility<'a, D: Delay>(
        &mut self,
        visibility: Cursor,
        delay: &'a mut D,
    ) -> Result<()> {
        self.display_mode.cursor_visibility = visibility;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay).await?;

        Ok(())
    }

    /// Set if the characters on the display should be visible
    pub async fn set_display<'a, D: Delay>(
        &mut self,
        display: Display,
        delay: &'a mut D,
    ) -> Result<()> {
        self.display_mode.display = display;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay).await?;

        Ok(())
    }

    /// Set if the cursor should blink
    pub async fn set_cursor_blink<'a, D: Delay>(
        &mut self,
        blink: CursorBlink,
        delay: &'a mut D,
    ) -> Result<()> {
        self.display_mode.cursor_blink = blink;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay).await?;

        Ok(())
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
    pub async fn set_cursor_mode<'a, D: Delay>(
        &mut self,
        mode: CursorMode,
        delay: &'a mut D,
    ) -> Result<()> {
        self.entry_mode.cursor_mode = mode;

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd, delay).await?;

        Ok(())
    }

    /// Set the cursor position
    ///
    /// ```rust,ignore
    /// // Move to line 2
    /// lcd.set_cursor_pos(40)
    /// ```
    pub async fn set_cursor_pos<'a, D: Delay>(
        &mut self,
        position: u8,
        delay: &'a mut D,
    ) -> Result<()> {
        let lower_7_bits = 0b0111_1111 & position;

        self.write_command(0b1000_0000 | lower_7_bits, delay)
            .await?;

        Ok(())
    }

    /// Shift just the cursor to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_cursor(Direction::Left);
    /// lcd.shift_cursor(Direction::Right);
    /// ```
    pub async fn shift_cursor<'a, D: Delay>(
        &mut self,
        dir: Direction,
        delay: &'a mut D,
    ) -> Result<()> {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_0000 | bits | bits, delay).await?;

        Ok(())
    }

    /// Shift the entire display to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_display(Direction::Left);
    /// lcd.shift_display(Direction::Right);
    /// ```
    pub async fn shift_display<'a, D: Delay>(
        &mut self,
        dir: Direction,
        delay: &'a mut D,
    ) -> Result<()> {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_1000 | bits, delay).await?;

        Ok(())
    }

    /// Write a single character to the `HD44780`. This `char` just gets downcast to a `u8`
    /// internally, so make sure that whatever character you're printing fits inside that range, or
    /// you can just use [write_byte](#method.write_byte) to have the compiler check for you.
    /// See the documentation on that function for more details about compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_char('A', &'a mut Delay)?; // prints 'A'
    /// ```
    pub async fn write_char<'a, D: Delay>(&mut self, data: char, delay: &'a mut D) -> Result<()> {
        self.write_byte(data as u8, delay).await
    }

    async fn write_command<'a, D: Delay>(&mut self, cmd: u8, delay: &'a mut D) -> Result<()> {
        self.bus.write(cmd, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;
        Ok(())
    }

    async fn init_4bit<'a, D: Delay>(&mut self, delay: &'a mut D) -> Result<()> {
        // Wait for the LCD to wakeup if it was off
        delay.delay_ms(15u8 as u64).await;

        // Initialize Lcd in 4-bit mode
        self.bus.write(0x33, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_ms(5u8 as u64).await;

        // Sets 4-bit operation and enables 5x7 mode for chars
        self.bus.write(0x32, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        self.bus.write(0x28, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Clear Display
        self.bus.write(0x0E, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Move the cursor to beginning of first line
        self.bus.write(0x01, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Set entry mode
        self.bus
            .write(self.entry_mode.as_byte(), false, delay)
            .await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        self.bus.write(0x80, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        Ok(())
    }

    // Follow the 8-bit setup procedure as specified in the HD44780 datasheet
    async fn init_8bit<'a, D: Delay>(&mut self, delay: &'a mut D) -> Result<()> {
        // Wait for the LCD to wakeup if it was off
        delay.delay_ms(15u8 as u64).await;

        // Initialize Lcd in 8-bit mode
        self.bus.write(0b0011_0000, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_ms(5u8 as u64).await;

        // Sets 8-bit operation and enables 5x7 mode for chars
        self.bus.write(0b0011_1000, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        self.bus.write(0b0000_1110, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Clear Display
        self.bus.write(0b0000_0001, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Move the cursor to beginning of first line
        self.bus.write(0b000_0111, false, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        // Set entry mode
        self.bus
            .write(self.entry_mode.as_byte(), false, delay)
            .await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        Ok(())
    }

    /// Writes a string to the HD44780. Internally, this just prints the string byte-by-byte, so
    /// make sure the characters in the string fit in a normal `u8`. See the documentation on
    /// [write_byte](#method.write_byte) for more details on compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_str("Hello, World!", &'a mut Delay)?;
    /// ```
    pub async fn write_str<'a, D: Delay>(&mut self, string: &str, delay: &'a mut D) -> Result<()> {
        self.write_bytes(string.as_bytes(), delay).await
    }

    /// Writes a sequence of bytes to the HD44780. See the documentation on the
    /// [write_byte](#method.write_byte) function for more details about compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_bytes(b"Hello, World!", &'a mut Delay)?;
    /// ```
    pub async fn write_bytes<'a, D: Delay>(
        &mut self,
        string: &[u8],
        delay: &'a mut D,
    ) -> Result<()> {
        for &b in string {
            self.write_byte(b, delay).await?;
        }
        Ok(())
    }

    /// Writes a single byte to the HD44780. These usually map to ASCII characters when printed on the
    /// screen, but not always. While it varies depending on the ROM of the LCD, `0x20u8..=0x5b`
    /// and `0x5d..=0x7d` should map to their standard ASCII characters. That is, all the printable
    /// ASCII characters work, excluding `\` and `~`, which are usually displayed as `¥` and `🡢`
    /// respectively.
    ///
    /// More information can be found in the Hitachi datasheets for the HD44780.
    ///
    /// ```rust,ignore
    /// lcd.write_byte(b'A', &'a mut Delay)?; // prints 'A'
    /// lcd.write_byte(b'\\', &'a mut Delay)?; // usually prints ¥
    /// lcd.write_byte(b'~', &'a mut Delay)?; // usually prints 🡢
    /// lcd.write_byte(b'\x7f', &'a mut Delay)?; // usually prints 🡠
    /// ```
    pub async fn write_byte<'a, D: Delay>(&mut self, data: u8, delay: &'a mut D) -> Result<()> {
        self.bus.write(data, true, delay).await?;

        // Wait for the command to be processed
        delay.delay_us(100 as u64).await;

        Ok(())
    }

    // Pulse the enable pin telling the HD44780 that we something for it
    /*fn pulse_enable(&mut self) {
        self.en.set_high();
        self.delay.delay_ms(15u8);
        self.en.set_low();
    }*/
}

//impl<B> Write for HD44780<B>
//where
//    B: DataBus,
//{
//    fn write_str(&mut self, string: &str) -> Result {
//        for c in string.chars() {
//            self.write_char(c);
//        }
//        Ok(())
//    }
//}
