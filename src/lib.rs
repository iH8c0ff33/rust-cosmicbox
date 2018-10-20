extern crate hidapi;

use error::CosmicBoxResult;

pub mod error;
pub mod hid;

pub const VENDOR_ID: u16 = 0x0fc5;
pub const PRODUCT_ID: u16 = 0xb080;

#[derive(Debug, PartialEq)]
pub struct TriggerOptions {
    pub top: bool,
    pub bottom: bool,
    pub ext: bool,
}

pub struct CosmicBox<T> {
    device: T,
}

pub struct Counters {
    pub top: u16,
    pub bottom: u16,
    pub ext: u16,
    pub coinc: u16
}

pub trait GenericCosmicBox<T> {
    fn new(T) -> Self;
    fn set_trigger(&self, options: &TriggerOptions) -> CosmicBoxResult<()>;
    fn get_trigger(&self) -> CosmicBoxResult<TriggerOptions>;
    fn reset(&self) -> CosmicBoxResult<()>;
    fn set_address(&self, u8) -> CosmicBoxResult<()>;
    fn get_counters(&self) -> CosmicBoxResult<Counters>;
}

#[cfg(test)]
mod tests {}
