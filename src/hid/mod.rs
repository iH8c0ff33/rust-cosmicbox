use hidapi::{HidApi, HidDevice};

use {CosmicBox, PRODUCT_ID, VENDOR_ID};

pub mod logic;
pub mod packet;
pub mod proto;

impl CosmicBox<HidDevice> {
    pub fn connect(hid: HidApi) -> Self {
        Self {
            device: hid.open(VENDOR_ID, PRODUCT_ID).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use hidapi::HidApi;

    use CosmicBox;

    #[test]
    fn cosmicbox_connect() {
        let _ = CosmicBox::connect(HidApi::new().unwrap());
    }
}
