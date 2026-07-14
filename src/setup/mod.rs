use crate::{
	bus::{EightBitBusPins, FourBitBusPins},
	charset::{CharsetUniversal, CharsetWithFallback, EmptyFallback},
	entry_mode::EntryMode,
	memory_map::DisplayMemoryMap,
};

pub(crate) mod blocking;

#[cfg(feature = "async")]
pub(crate) mod non_blocking;

/// Placeholder until the pin/bus is specified.
#[derive(Debug, Clone, Copy)]
pub struct Unspecified;

#[derive(Clone, Copy)]
pub struct DisplayOptions8Bit<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
	/// Memory map used for mapping 2D coordinates to the display.
	pub memory_map: M,
	/// The character set this display uses.
	pub charset: C,
	pub entry_mode: EntryMode,
	pub pins: EightBitBusPins<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>,
}

impl<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> core::fmt::Debug
	for DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
where
	M: DisplayMemoryMap + core::fmt::Debug,
	C: CharsetWithFallback + core::fmt::Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("DisplayOptions4Bit")
			.field("memory_map", &self.memory_map)
			.field("charset", &self.charset)
			.field("entry_mode", &self.entry_mode)
			.field("pins", &self.pins)
			.finish()
	}
}

#[derive(Clone, Copy)]
pub struct DisplayOptions4Bit<M: DisplayMemoryMap, C: CharsetWithFallback, RS, EN, D4, D5, D6, D7> {
	/// Memory map used for mapping 2D coordinates to the display.
	pub memory_map: M,
	/// The character set this display uses.
	pub charset: C,
	pub entry_mode: EntryMode,
	pub pins: FourBitBusPins<RS, EN, D4, D5, D6, D7>,
}

impl<M, C, RS, EN, D4, D5, D6, D7> core::fmt::Debug for DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
where
	M: DisplayMemoryMap + core::fmt::Debug,
	C: CharsetWithFallback + core::fmt::Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("DisplayOptions4Bit")
			.field("memory_map", &self.memory_map)
			.field("charset", &self.charset)
			.field("entry_mode", &self.entry_mode)
			.field("pins", &self.pins)
			.finish()
	}
}

pub struct DisplayOptionsI2C<M: DisplayMemoryMap, C: CharsetWithFallback, I2C> {
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
			pins: EightBitBusPins {
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
			},
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
			pins: FourBitBusPins {
				rs: Unspecified,
				en: Unspecified,
				d4: Unspecified,
				d5: Unspecified,
				d6: Unspecified,
				d7: Unspecified,
			},
		}
	}
}

impl<M: DisplayMemoryMap> DisplayOptionsI2C<M, EmptyFallback<CharsetUniversal>, Unspecified> {
	pub fn new(memory_map: M) -> Self {
		Self {
			memory_map,
			charset: CharsetUniversal::EMPTY_FALLBACK,
			entry_mode: EntryMode::default(),
			i2c_bus: Unspecified,
			address: 0,
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

			pub fn with_entry_mode(mut self, entry_mode: EntryMode) -> Self {
				self.entry_mode = entry_mode;
				self
			}
		}
	};
}

builder_functions!(DisplayOptions8Bit < RS, EN, D0, D1, D2, D3, D4, D5, D6, D7 > { pins });
builder_functions!(DisplayOptions4Bit < RS, EN, D4, D5, D6, D7 > { pins });
builder_functions!(DisplayOptionsI2C<I2C> { i2c_bus, address });

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
		pins: EightBitBusPins<RS2, EN2, D02, D12, D22, D32, D42, D52, D62, D72>,
	) -> DisplayOptions8Bit<M, C, RS2, EN2, D02, D12, D22, D32, D42, D52, D62, D72> {
		DisplayOptions8Bit { memory_map: self.memory_map, charset: self.charset, entry_mode: self.entry_mode, pins }
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
		pins: FourBitBusPins<RS2, EN2, D42, D52, D62, D72>,
	) -> DisplayOptions4Bit<M, C, RS2, EN2, D42, D52, D62, D72> {
		DisplayOptions4Bit { memory_map: self.memory_map, charset: self.charset, entry_mode: self.entry_mode, pins }
	}
}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, I2C> DisplayOptionsI2C<M, C, I2C> {
	pub fn with_i2c_bus<I2C2>(self, i2c_bus: I2C2, address: u8) -> DisplayOptionsI2C<M, C, I2C2> {
		DisplayOptionsI2C {
			memory_map: self.memory_map,
			charset: self.charset,
			entry_mode: EntryMode::default(),
			i2c_bus,
			address,
		}
	}
}
