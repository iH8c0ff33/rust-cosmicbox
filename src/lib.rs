extern crate hidapi;

use error::CosmicboxResult;

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

pub enum Counter {
    Top,
    Bottom,
    Ext,
    Coinc,
}

pub trait GenericCosmicBox<T> {
    fn new(T) -> Self;
    fn set_trigger(&self, options: &TriggerOptions);
    fn get_trigger(&self) -> TriggerOptions;
    fn reset(&self);
    fn set_address(&self, u8);
    fn get_count(&self, Counter) -> CosmicboxResult<u16>;
}

#[cfg(test)]
mod tests {}
