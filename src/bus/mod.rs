use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub mod eightbit;
pub mod fourbit;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;

pub trait DataBus {
    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D);
    fn init<D: DelayUs<u16> + DelayMs<u8>>(&mut self, entry_mode: u8, delay: &mut D);

    // TODO
    // fn read(...)
}
