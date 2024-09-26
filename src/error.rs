#[derive(Debug)]
pub enum Error<IoE> {
	/// Error related to IO of the MCU.
	Io {
		/// Which port (pin or interface) the error belongs to.
		port: Port,
		error: IoE,
	},
	/// Invalid coordinates on the display.
	Position { position: (u8, u8), size: (u8, u8) },
}

impl<E> Error<E> {
	pub(crate) const fn wrap_io(port: Port) -> impl FnOnce(E) -> Self {
		move |error| Self::Io { port, error }
	}
}

impl<E: core::fmt::Debug> core::fmt::Display for Error<E> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Io { port, error } => write!(f, "error on {port:?}: {error:?}"),
			Self::Position { position, size } => write!(
				f,
				"coordinates out of bounds: ({};{}) not fitting in a {}x{} display",
				position.0, position.1, size.0, size.1
			),
		}
	}
}

#[cfg(feature = "defmt")]
impl<E: defmt::Format> defmt::Format for Error<E> {
	fn format(&self, fmt: defmt::Formatter) {
		match self {
			Self::Io { port, error } => defmt::write!(fmt, "error on {:?}: {:?}", port, error),
			Self::Position { position, size } => defmt::write!(
				fmt,
				"coordinates out of bounds: ({};{}) not fitting in a {}x{} display",
				position.0,
				position.1,
				size.0,
				size.1
			),
		}
	}
}

#[cfg(feature = "ufmt")]
impl<E: ufmt::uDebug> ufmt::uDisplay for Error<E> {
	fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> core::result::Result<(), W::Error>
	where
		W: ufmt::uWrite + ?Sized,
	{
		match self {
			Self::Io { port, error } => ufmt::uwrite!(f, "error on {:?}: {:?}", port, error),
			Self::Position { position, size } => ufmt::uwrite!(
				f,
				"coordinates out of bounds: ({};{}) not fitting in a {}x{} display",
				position.0,
				position.1,
				size.0,
				size.1
			),
		}
	}
}

impl<E: core::error::Error + 'static> core::error::Error for Error<E> {
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Io { error, .. } => Some(error),
			_ => None,
		}
	}
}

pub type Result<T, E> = core::result::Result<T, Error<E>>;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub enum Port {
	/// Pin `D0` of an [EightBitBus][`crate::bus::EightBitBus`].
	D0,
	/// Pin `D1` of an [EightBitBus][`crate::bus::EightBitBus`].
	D1,
	/// Pin `D2` of an [EightBitBus][`crate::bus::EightBitBus`].
	D2,
	/// Pin `D3` of an [EightBitBus][`crate::bus::EightBitBus`].
	D3,
	/// Pin `D4` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	D4,
	/// Pin `D5` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	D5,
	/// Pin `D6` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	D6,
	/// Pin `D7` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	D7,
	/// Pin `RS` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	RS,
	/// Pin `EN` of a [FourBitBus][`crate::bus::FourBitBus`] or
	/// [EightBitBus][`crate::bus::EightBitBus`].
	EN,
	/// [I2CBus][`crate::bus::I2CBus`].
	I2C,
}
