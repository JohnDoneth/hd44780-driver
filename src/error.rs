#[derive(Debug)]
pub struct Error;
pub type Result<T> = core::result::Result<T, Error>;
