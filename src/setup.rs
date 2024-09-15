use embedded_hal::{
	delay::DelayNs,
	digital::{self, OutputPin},
	i2c::I2c,
};

use crate::{
	bus::{DataBus, EightBitBus, FourBitBus, I2CBus},
	charset::{CharsetUniversal, CharsetWithFallback, EmptyFallback},
	entry_mode::EntryMode,
	error::Error,
	memory_map::DisplayMemoryMap,
	DisplayMode, HD44780,
};

mod sealed {
	use embedded_hal::delay::DelayNs;

	use crate::{bus::DataBus, charset::CharsetWithFallback, error::Error, memory_map::DisplayMemoryMap, HD44780};

	pub trait SealedDisplayOptions: Sized {
		type Bus: DataBus;
		type MemoryMap: DisplayMemoryMap;
		type Charset: CharsetWithFallback;
		type IoError: core::fmt::Debug;

		fn new_display<D: DelayNs>(
			self,
			delay: &mut D,
		) -> Result<HD44780<Self::Bus, Self::MemoryMap, Self::Charset>, (Self, Error<Self::IoError>)>;
	}
}

/// Use this as an argument to [`HD44780::new`].
/// - [`DisplayOptionsI2C`]
/// - [`DisplayOptions4Bit`]
/// - [`DisplayOptions8Bit`]
pub trait DisplayOptions: sealed::SealedDisplayOptions {}

/// Placeholder until the pin is specified.
#[derive(Debug, Clone, Copy)]
pub struct Unspecified;

#[derive(Debug, Clone, Copy)]
pub struct DisplayOptions8Bit<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
	/// Memory map used for mapping 2D coordinates to the display.
	pub memory_map: M,
	/// The character set this display uses.
	pub charset: C,
	pub entry_mode: EntryMode,
	pub rs: RS,
	pub en: EN,
	pub d0: D0,
	pub d1: D1,
	pub d2: D2,
	pub d3: D3,
	pub d4: D4,
	pub d5: D5,
	pub d6: D6,
	pub d7: D7,
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayOptions4Bit<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D4, D5, D6, D7> {
	/// Memory map used for mapping 2D coordinates to the display.
	pub memory_map: M,
	/// The character set this display uses.
	pub charset: C,
	pub entry_mode: EntryMode,
	pub rs: RS,
	pub en: EN,
	pub d4: D4,
	pub d5: D5,
	pub d6: D6,
	pub d7: D7,
}

pub struct DisplayOptionsI2C<M: DisplayMemoryMap, C: CharsetWithFallback, I2C: I2c> {
	/// Memory map used for mapping 2D coordinates to the display.
	pub memory_map: M,
	/// The character set this display uses.
	pub charset: C,
	pub entry_mode: EntryMode,
	pub i2c_bus: I2C,
	pub address: u8,
}

impl<M: DisplayMemoryMap>
	DisplayOptions8Bit<
		M,
		EmptyFallback<CharsetUniversal>,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
	>
{
	pub fn new(memory_map: M) -> Self {
		Self {
			memory_map,
			charset: CharsetUniversal::EMPTY_FALLBACK,
			entry_mode: EntryMode::default(),
			rs: Unspecified,
			en: Unspecified,
			d0: Unspecified,
			d1: Unspecified,
			d2: Unspecified,
			d3: Unspecified,
			d4: Unspecified,
			d5: Unspecified,
			d6: Unspecified,
			d7: Unspecified,
		}
	}
}

impl<M: DisplayMemoryMap>
	DisplayOptions4Bit<
		M,
		EmptyFallback<CharsetUniversal>,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
		Unspecified,
	>
{
	pub fn new(memory_map: M) -> Self {
		Self {
			memory_map,
			charset: CharsetUniversal::EMPTY_FALLBACK,
			entry_mode: EntryMode::default(),
			rs: Unspecified,
			en: Unspecified,
			d4: Unspecified,
			d5: Unspecified,
			d6: Unspecified,
			d7: Unspecified,
		}
	}
}

impl<M: DisplayMemoryMap, I2C: I2c> DisplayOptionsI2C<M, EmptyFallback<CharsetUniversal>, I2C> {
	pub fn new(memory_map: M, i2c_bus: I2C, address: u8) -> Self {
		Self {
			memory_map,
			charset: CharsetUniversal::EMPTY_FALLBACK,
			entry_mode: EntryMode::default(),
			i2c_bus,
			address,
		}
	}
}

macro_rules! builder_functions {
	(
		$Options:ident < $($Gn:ident$(: $Gt:tt)?),* > { $($fn:ident),* }
	) => {
		impl<M: DisplayMemoryMap, C: CharsetWithFallback, $($Gn$(: $Gt)?),*> $Options<M, C, $($Gn),*> {
			pub fn with_memory_map<M2: DisplayMemoryMap>(self, memory_map: M2) -> $Options<M2, C, $($Gn),*> {
				$Options {
					memory_map,
					charset: self.charset,
					entry_mode: self.entry_mode,
					$($fn: self.$fn),*
				}
			}

			pub fn with_charset<C2: CharsetWithFallback>(self, charset: C2) -> $Options<M, C2, $($Gn),*> {
				$Options {
					memory_map: self.memory_map,
					charset,
					entry_mode: self.entry_mode,
					$($fn: self.$fn),*
				}
			}
		}
	};
}

builder_functions!(DisplayOptions8Bit<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> { rs, en, d0, d1, d2, d3, d4, d5, d6, d7 });
builder_functions!(DisplayOptions4Bit<RS, EN, D4, D5, D6, D7> { rs, en, d4, d5, d6, d7 });
builder_functions!(DisplayOptionsI2C<I2C: I2c> { i2c_bus, address });

impl<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
	DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	/// The eight d0..d7 pins are used to send and recieve with
	/// the `HD44780`.
	/// The register select pin `rs` is used to tell the `HD44780`
	/// if incoming data is a command or data.
	/// The enable pin `en` is used to tell the `HD44780` that there
	/// is data on the 8 data pins and that it should read them in.
	pub fn with_pins<RS2, EN2, D02, D12, D22, D32, D42, D52, D62, D72>(
		self,
		rs: RS2,
		en: EN2,
		d0: D02,
		d1: D12,
		d2: D22,
		d3: D32,
		d4: D42,
		d5: D52,
		d6: D62,
		d7: D72,
	) -> DisplayOptions8Bit<M, C, RS2, EN2, D02, D12, D22, D32, D42, D52, D62, D72> {
		DisplayOptions8Bit {
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: self.entry_mode,
			rs,
			en,
			d0,
			d1,
			d2,
			d3,
			d4,
			d5,
			d6,
			d7,
		}
	}
}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D4, D5, D6, D7>
	DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
{
	/// The four d4..d7 pins are used to send and recieve with
	/// the `HD44780`.
	/// The register select pin `rs` is used to tell the `HD44780`
	/// if incoming data is a command or data.
	/// The enable pin `en` is used to tell the `HD44780` that there
	/// is data on the 4 data pins and that it should read them in.
	pub fn with_pins<RS2, EN2, D42, D52, D62, D72>(
		self,
		rs: RS2,
		en: EN2,
		d4: D42,
		d5: D52,
		d6: D62,
		d7: D72,
	) -> DisplayOptions4Bit<M, C, RS2, EN2, D42, D52, D62, D72> {
		DisplayOptions4Bit {
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: self.entry_mode,
			rs,
			en,
			d4,
			d5,
			d6,
			d7,
		}
	}
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> DisplayOptions for DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> sealed::SealedDisplayOptions for DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	type Bus = EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	fn new_display<D: DelayNs>(
		mut self,
		delay: &mut D,
	) -> Result<HD44780<Self::Bus, Self::MemoryMap, Self::Charset>, (Self, Error<Self::IoError>)> {
		let mut bus = EightBitBus::from_pins(
			self.rs, self.en, self.d0, self.d1, self.d2, self.d3, self.d4, self.d5, self.d6, self.d7,
		);

		if let Err(error) = init_8bit(&mut bus, &self.entry_mode, delay) {
			let pins = bus.destroy();
			self.rs = pins.0;
			self.en = pins.1;
			self.d0 = pins.2;
			self.d1 = pins.3;
			self.d2 = pins.4;
			self.d3 = pins.5;
			self.d4 = pins.6;
			self.d5 = pins.7;
			self.d6 = pins.8;
			self.d7 = pins.9;
			return Err((self, error));
		}

		Ok(HD44780 {
			bus,
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: self.entry_mode,
			display_mode: DisplayMode::default(),
		})
	}
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> DisplayOptions for DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> sealed::SealedDisplayOptions for DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
{
	type Bus = FourBitBus<RS, EN, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	fn new_display<D: DelayNs>(
		mut self,
		delay: &mut D,
	) -> Result<HD44780<Self::Bus, Self::MemoryMap, Self::Charset>, (Self, Error<Self::IoError>)> {
		let mut bus = FourBitBus::from_pins(self.rs, self.en, self.d4, self.d5, self.d6, self.d7);

		if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay) {
			let pins = bus.destroy();
			self.rs = pins.0;
			self.en = pins.1;
			self.d4 = pins.2;
			self.d5 = pins.3;
			self.d6 = pins.4;
			self.d7 = pins.5;
			return Err((self, error));
		}

		Ok(HD44780 {
			bus,
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: self.entry_mode,
			display_mode: DisplayMode::default(),
		})
	}
}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, I2C: I2c> DisplayOptions for DisplayOptionsI2C<M, C, I2C> {}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, I2C: I2c> sealed::SealedDisplayOptions
	for DisplayOptionsI2C<M, C, I2C>
{
	type Bus = I2CBus<I2C>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = I2C::Error;

	fn new_display<D: DelayNs>(
		mut self,
		delay: &mut D,
	) -> Result<HD44780<Self::Bus, Self::MemoryMap, Self::Charset>, (Self, Error<Self::IoError>)> {
		let mut bus = I2CBus::new(self.i2c_bus, self.address);

		if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay) {
			self.i2c_bus = bus.destroy();
			return Err((self, error));
		}

		Ok(HD44780 {
			bus,
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: self.entry_mode,
			display_mode: DisplayMode::default(),
		})
	}
}

// Follow the 8-bit setup procedure as specified in the HD44780 datasheet
fn init_8bit<B: DataBus, D: DelayNs>(
	bus: &mut B,
	entry_mode: &EntryMode,
	delay: &mut D,
) -> Result<(), Error<B::Error>> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15u32);

	// Initialize Lcd in 8-bit mode
	bus.write(0b0011_0000, false, delay)?;

	// Wait for the command to be processed
	delay.delay_ms(5u32);

	// Sets 8-bit operation and enables 5x7 mode for chars
	bus.write(0b0011_1000, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0b0000_1110, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Clear Display
	bus.write(0b0000_0001, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Move the cursor to beginning of first line
	bus.write(0b000_0111, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	Ok(())
}

fn init_4bit<B: DataBus, D: DelayNs>(
	bus: &mut B,
	entry_mode: &EntryMode,
	delay: &mut D,
) -> Result<(), Error<B::Error>> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15u32);

	// Initialize Lcd in 4-bit mode
	bus.write(0x33, false, delay)?;

	// Wait for the command to be processed
	delay.delay_ms(5u32);

	// Sets 4-bit operation and enables 5x7 mode for chars
	bus.write(0x32, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0x28, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Clear Display
	bus.write(0x0E, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Move the cursor to beginning of first line
	bus.write(0x01, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0x80, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	Ok(())
}
