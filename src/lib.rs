extern crate hidapi;

pub mod hid;

pub const VENDOR_ID: u16 = 0x0fc5;
pub const PRODUCT_ID: u16 = 0xb080;

#[derive(Debug, PartialEq)]
pub struct TriggerOptions {
    top: bool,
    bottom: bool,
    ext: bool,
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
    fn get_count(&self, Counter) -> u16;
}

#[cfg(test)]
mod tests {}
