#![no_std]
#![cfg_attr(feature = "async", feature(type_alias_impl_trait))]
#![cfg_attr(feature = "async", feature(impl_trait_in_assoc_type))]

use core::convert::Infallible;

use charset::CharsetWithFallback;
use display_size::DisplaySize;
use embedded_hal::delay::DelayNs;

pub mod bus;
use bus::DataBus;

pub mod error;
use error::{Error, Result};

pub mod entry_mode;

use entry_mode::{CursorMode, EntryMode};

pub mod setup;

pub mod charset;

pub mod memory_map;

pub mod display_mode;
pub mod display_size;

pub use display_mode::DisplayMode;
use memory_map::DisplayMemoryMap;
use setup::DisplayOptions;

/// Implementation of async functionality
#[cfg(feature = "async")]
pub mod non_blocking;

pub struct HD44780<B: DataBus, M: DisplayMemoryMap, C: CharsetWithFallback> {
	bus: B,
	memory_map: M,
	charset: C,
	entry_mode: EntryMode,
	display_mode: DisplayMode,
}

/// Used in the direction argument for shifting the cursor and the display
pub enum Direction {
	Left,
	Right,
}

/// Used in set_display_mode to make the parameters more clear
pub enum Display {
	On,
	Off,
}

pub enum Cursor {
	Visible,
	Invisible,
}

pub enum CursorBlink {
	On,
	Off,
}

// impl<
// 		M: DisplayMemoryMap,
// 		C: CharsetWithFallback,
// 		RS: OutputPin<Error = E>,
// 		EN: OutputPin<Error = E>,
// 		D0: OutputPin<Error = E>,
// 		D1: OutputPin<Error = E>,
// 		D2: OutputPin<Error = E>,
// 		D3: OutputPin<Error = E>,
// 		D4: OutputPin<Error = E>,
// 		D5: OutputPin<Error = E>,
// 		D6: OutputPin<Error = E>,
// 		D7: OutputPin<Error = E>,
// 		E: digital::Error,
// 	> HD44780<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>, M, C>
// {
// 	/// Create an instance of a `HD44780` from 8 data pins, a register select
// 	/// pin, an enable pin and a struct implementing the delay trait.
// 	/// - The delay instance is used to sleep between commands to
// 	/// ensure the `HD44780` has enough time to process commands.
// 	/// - The eight db0..db7 pins are used to send and recieve with
// 	///  the `HD44780`.
// 	/// - The register select pin is used to tell the `HD44780`
// 	/// if incoming data is a command or data.
// 	/// - The enable pin is used to tell the `HD44780` that there
// 	/// is data on the 8 data pins and that it should read them in.
// 	///
// 	#[allow(clippy::type_complexity)]
// 	pub fn new_8bit<D: DelayNs>(
// 		rs: RS,
// 		en: EN,
// 		d0: D0,
// 		d1: D1,
// 		d2: D2,
// 		d3: D3,
// 		d4: D4,
// 		d5: D5,
// 		d6: D6,
// 		d7: D7,
// 		delay: &mut D,
// 	) -> Result<HD44780<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>, M, C>, E> {
// 		let mut hd = HD44780 {
// 			bus: EightBitBus::from_pins(rs, en, d0, d1, d2, d3, d4, d5, d6, d7),
// 			entry_mode: EntryMode::default(),
// 			display_mode: DisplayMode::default(),
// 			display_size: DisplaySize::default(),
// 		};

// 		hd.init_8bit(delay)?;

// 		Ok(hd)
// 	}
// }

// impl<
// 		RS: OutputPin<Error = E>,
// 		EN: OutputPin<Error = E>,
// 		D4: OutputPin<Error = E>,
// 		D5: OutputPin<Error = E>,
// 		D6: OutputPin<Error = E>,
// 		D7: OutputPin<Error = E>,
// 		E: digital::Error,
// 	> HD44780<FourBitBus<RS, EN, D4, D5, D6, D7>>
// {
// 	/// Create an instance of a `HD44780` from 4 data pins, a register select
// 	/// pin, an enable pin and a struct implementing the delay trait.
// 	/// - The delay instance is used to sleep between commands to
// 	/// ensure the `HD44780` has enough time to process commands.
// 	/// - The four db0..db3 pins are used to send and recieve with
// 	///  the `HD44780`.
// 	/// - The register select pin is used to tell the `HD44780`
// 	/// if incoming data is a command or data.
// 	/// - The enable pin is used to tell the `HD44780` that there
// 	/// is data on the 4 data pins and that it should read them in.
// 	///
// 	/// This mode operates differently than 8 bit mode by using 4 less
// 	/// pins for data, which is nice on devices with less I/O although
// 	/// the I/O takes a 'bit' longer
// 	///
// 	/// Instead of commands being sent byte by byte each command is
// 	/// broken up into it's upper and lower nibbles (4 bits) before
// 	/// being sent over the data bus
// 	///
// 	#[allow(clippy::type_complexity)]
// 	pub fn new_4bit<D: DelayNs>(
// 		rs: RS,
// 		en: EN,
// 		d4: D4,
// 		d5: D5,
// 		d6: D6,
// 		d7: D7,
// 		delay: &mut D,
// 	) -> Result<HD44780<FourBitBus<RS, EN, D4, D5, D6, D7>>, E> {
// 		let mut hd = HD44780 {
// 			bus: FourBitBus::from_pins(rs, en, d4, d5, d6, d7),
// 			entry_mode: EntryMode::default(),
// 			display_mode: DisplayMode::default(),
// 			display_size: DisplaySize::default(),
// 		};

// 		hd.init_4bit(delay)?;

// 		Ok(hd)
// 	}
// }

// impl<I2C: I2c> HD44780<I2CBus<I2C>> {
// 	/// Create an instance of a `HD44780` from an i2c write peripheral,
// 	/// the `HD44780` I2C address and a struct implementing the delay trait.
// 	/// - The delay instance is used to sleep between commands to
// 	/// ensure the `HD44780` has enough time to process commands.
// 	/// - The i2c peripheral is used to send data to the `HD44780` and to set
// 	/// its register select and enable pins.
// 	///
// 	/// This mode operates on an I2C bus, using an I2C to parallel port expander
// 	///
// 	pub fn new_i2c<D: DelayNs>(i2c_bus: I2C, address: u8, delay: &mut D) -> Result<HD44780<I2CBus<I2C>>, I2C::Error> {
// 		let mut hd = HD44780 {
// 			bus: I2CBus::new(i2c_bus, address),
// 			entry_mode: EntryMode::default(),
// 			display_mode: DisplayMode::default(),
// 			display_size: DisplaySize::default(),
// 		};

// 		hd.init_4bit(delay)?;

// 		Ok(hd)
// 	}
// }

impl<B, M, C> HD44780<B, M, C>
where
	B: DataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	/// Create an instance of a `HD44780` using a struct implementing
	/// the delay trait.
	/// The delay instance is used to sleep between commands to
	/// ensure the `HD44780` has enough time to process commands.
	///
	/// If there was an error when setting up the display, the settings
	/// are returned as a tuple together with the error. This can be used
	/// to retry on error, or just to get back access to registers or buses.
	pub fn new<Opt, D: DelayNs>(options: Opt, delay: &mut D) -> core::result::Result<Self, (Opt, Error<Opt::IoError>)>
	where
		Opt: DisplayOptions<Bus = B, MemoryMap = M, Charset = C>,
	{
		options.new_display(delay)
	}

	pub fn destroy(self) -> B {
		self.bus
	}

	/// Unshifts the display and sets the cursor position to 0
	///
	/// ```rust,ignore
	/// lcd.reset();
	/// ```
	pub fn reset<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), B::Error> {
		self.write_command(0b0000_0010, delay)?;

		Ok(())
	}

	/// Set if the display should be on, if the cursor should be
	/// visible, and if the cursor should blink
	///
	/// Note: This is equivilent to calling all of the other relavent
	/// methods however this operation does it all in one go to the `HD44780`
	pub fn set_display_mode<D: DelayNs>(&mut self, display_mode: DisplayMode, delay: &mut D) -> Result<(), B::Error> {
		self.display_mode = display_mode;

		let cmd_byte = self.display_mode.as_byte();

		self.write_command(cmd_byte, delay)?;

		Ok(())
	}

	/// Clear the entire display
	///
	/// ```rust,ignore
	/// lcd.clear();
	/// ```
	pub fn clear<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), B::Error> {
		self.write_command(0b0000_0001, delay)?;

		Ok(())
	}

	/// Get the memory map information for this display.
	pub fn memory_map(&self) -> &M {
		&self.memory_map
	}

	/// Get the display size.
	pub fn display_size(&self) -> DisplaySize {
		self.memory_map.display_size()
	}

	/// If enabled, automatically scroll the display when a new
	/// character is written to the display
	///
	/// ```rust,ignore
	/// lcd.set_autoscroll(true);
	/// ```
	pub fn set_autoscroll<D: DelayNs>(&mut self, enabled: bool, delay: &mut D) -> Result<(), B::Error> {
		self.entry_mode.shift_mode = enabled.into();

		let cmd = self.entry_mode.as_byte();

		self.write_command(cmd, delay)?;

		Ok(())
	}

	/// Set if the cursor should be visible
	pub fn set_cursor_visibility<D: DelayNs>(&mut self, visibility: Cursor, delay: &mut D) -> Result<(), B::Error> {
		self.display_mode.cursor_visibility = visibility;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay)?;

		Ok(())
	}

	/// Set if the characters on the display should be visible
	pub fn set_display<D: DelayNs>(&mut self, display: Display, delay: &mut D) -> Result<(), B::Error> {
		self.display_mode.display = display;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay)?;

		Ok(())
	}

	/// Set if the cursor should blink
	pub fn set_cursor_blink<D: DelayNs>(&mut self, blink: CursorBlink, delay: &mut D) -> Result<(), B::Error> {
		self.display_mode.cursor_blink = blink;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay)?;

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
	pub fn set_cursor_mode<D: DelayNs>(&mut self, mode: CursorMode, delay: &mut D) -> Result<(), B::Error> {
		self.entry_mode.cursor_mode = mode;

		let cmd = self.entry_mode.as_byte();

		self.write_command(cmd, delay)?;

		Ok(())
	}

	/// Set the cursor position
	///
	/// ```rust,ignore
	/// // Move to the start of line 2
	/// // for a 20 columns display
	/// lcd.set_cursor_pos(40)
	/// ```
	pub fn set_cursor_pos<D: DelayNs>(&mut self, position: u8, delay: &mut D) -> Result<(), B::Error> {
		let size = self.display_size().get();
		let position = (position % size.0, position / size.0);
		self.set_cursor_xy(position, delay)
	}

	/// Set the cursor position
	///
	/// ```rust,ignore
	/// // Move to the start of line 3
	/// lcd.set_cursor_pos_xy(0,2)
	/// ```
	pub fn set_cursor_xy<D: DelayNs>(&mut self, position: (u8, u8), delay: &mut D) -> Result<(), B::Error> {
		let size = self.display_size().get();
		let Some(pos) = self.memory_map.address_for_xy(position.0, position.1) else {
			return Err(Error::Position { position, size });
		};

		self.write_command(0b1000_0000 | pos, delay)?;

		Ok(())
	}

	/// Shift just the cursor to the left or the right
	///
	/// ```rust,ignore
	/// lcd.shift_cursor(Direction::Left);
	/// lcd.shift_cursor(Direction::Right);
	/// ```
	pub fn shift_cursor<D: DelayNs>(&mut self, dir: Direction, delay: &mut D) -> Result<(), B::Error> {
		let bits = match dir {
			Direction::Left => 0b0000_0000,
			Direction::Right => 0b0000_0100,
		};

		self.write_command(0b0001_0000 | bits | bits, delay)?;

		Ok(())
	}

	/// Shift the entire display to the left or the right
	///
	/// ```rust,ignore
	/// lcd.shift_display(Direction::Left);
	/// lcd.shift_display(Direction::Right);
	/// ```
	pub fn shift_display<D: DelayNs>(&mut self, dir: Direction, delay: &mut D) -> Result<(), B::Error> {
		let bits = match dir {
			Direction::Left => 0b0000_0000,
			Direction::Right => 0b0000_0100,
		};

		self.write_command(0b0001_1000 | bits, delay)?;

		Ok(())
	}

	/// Write a single character to the `HD44780`. This `char` just gets downcast to a `u8`
	/// internally, so make sure that whatever character you're printing fits inside that range, or
	/// you can just use [write_byte](#method.write_byte) to have the compiler check for you.
	/// See the documentation on that function for more details about compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_char('A', &mut delay)?; // prints 'A'
	/// ```
	pub fn write_char<D: DelayNs>(&mut self, data: char, delay: &mut D) -> Result<(), B::Error> {
		self.write_byte(self.charset.code_from_utf8_with_fallback(data), delay)
	}

	fn write_command<D: DelayNs>(&mut self, cmd: u8, delay: &mut D) -> Result<(), B::Error> {
		self.bus.write(cmd, false, delay)?;

		// Wait for the command to be processed
		delay.delay_us(100);
		Ok(())
	}

	/// Writes a string to the HD44780. Internally, this just prints the string byte-by-byte, so
	/// make sure the characters in the string fit in a normal `u8`. See the documentation on
	/// [write_byte](#method.write_byte) for more details on compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_str("Hello, World!", &mut delay)?;
	/// ```
	pub fn write_str<D: DelayNs>(&mut self, string: &str, delay: &mut D) -> Result<(), B::Error> {
		for ch in string.chars() {
			self.write_char(ch, delay)?;
		}
		Ok(())
	}

	/// Writes a sequence of bytes to the HD44780. See the documentation on the
	/// [write_byte](#method.write_byte) function for more details about compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_bytes(b"Hello, World!", &mut delay)?;
	/// ```
	pub fn write_bytes<D: DelayNs>(&mut self, string: &[u8], delay: &mut D) -> Result<(), B::Error> {
		for &b in string {
			self.write_byte(b, delay)?;
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
	/// lcd.write_byte(b'A', &mut delay)?; // prints 'A'
	/// lcd.write_byte(b'\\', &mut delay)?; // usually prints ¥
	/// lcd.write_byte(b'~', &mut delay)?; // usually prints 🡢
	/// lcd.write_byte(b'\x7f', &mut delay)?; // usually prints 🡠
	/// ```
	pub fn write_byte<D: DelayNs>(&mut self, data: u8, delay: &mut D) -> Result<(), B::Error> {
		self.bus.write(data, true, delay)?;

		// Wait for the command to be processed
		delay.delay_us(100);

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
//            self.write_char(c, delay);
//        }
//        Ok(())
//    }
//}
