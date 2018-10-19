use std::thread;
use std::time::Duration;

use hidapi::HidDevice;

use error::CosmicboxResult;
use hid::packet::HidPacket;
use {CosmicBox, Counters, GenericCosmicBox, TriggerOptions};

impl GenericCosmicBox<HidDevice> for CosmicBox<HidDevice> {
    fn new(device: HidDevice) -> Self {
        Self { device }
    }

    fn set_trigger(&self, options: &TriggerOptions) {
        let packet = HidPacket::write_8(
            12,
            !((options.top as u8) << 2 | (options.bottom as u8) << 1 | (options.ext as u8)) << 5,
            (options.top as u8) << 7 | (options.bottom as u8) << 6 | (options.ext as u8) << 5,
        );

        self.send(packet).expect("couldn't send packet")
    }

    fn get_trigger(&self) -> TriggerOptions {
        let ports = self.read_8(100).expect("couldn't read packet");

        TriggerOptions {
            top: (ports[1] & (1 << 7)) != 0,
            bottom: (ports[1] & (1 << 6)) != 0,
            ext: (ports[1] & (1 << 5)) != 0,
        }
    }

    fn reset(&self) {
        self.send(HidPacket::write_8(12, 1 << 4, 0))
            .expect("couldn't send packet");
        thread::sleep(Duration::from_millis(1));
        self.send(HidPacket::write_8(12, 0, 1 << 4))
            .expect("couldn't send packet");
    }

    fn set_address(&self, address: u8) {
        self.send(HidPacket::write_8(12, !address & 0b111, address))
            .expect("couldn't set address")
    }

    fn get_counters(&self) -> CosmicboxResult<Counters> {
        let mut counters = Counters {
            top: 0,
            bottom: 0,
            ext: 0,
            coinc: 0,
        };

        self.set_address(0b000);
        let lsb = self.read_8(100)?[0];

        self.set_address(0b001);
        let msb = self.read_8(100)?[0];
        counters.top = (msb as u16) << 8 | lsb as u16;

        self.set_address(0b010);
        let lsb = self.read_8(100)?[0];

        self.set_address(0b011);
        let msb = self.read_8(100)?[0];
        counters.bottom = (msb as u16) << 8 | lsb as u16;

        self.set_address(0b100);
        let lsb = self.read_8(100)?[0];

        self.set_address(0b101);
        let msb = self.read_8(100)?[0];
        counters.ext = (msb as u16) << 8 | lsb as u16;

        self.set_address(0b110);
        let lsb = self.read_8(100)?[0];

        self.set_address(0b111);
        let msb = self.read_8(100)?[0];
        counters.coinc = (msb as u16) << 8 | lsb as u16;

        Ok(counters)
    }
}

#[cfg(test)]
mod tests {
    use {PRODUCT_ID, VENDOR_ID};

    use super::*;

    use hidapi::HidApi;

    #[test]
    fn cosmicbox_hid_new() {
        let hid = HidApi::new().expect("failed to get HID access");
        let device = hid
            .open(VENDOR_ID, PRODUCT_ID)
            .expect("failed to open HID port");

        let _ = CosmicBox::new(device);
    }

    #[test]
    fn cosmicbox_hid_set_get_trigger() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        let options = TriggerOptions {
            top: true,
            bottom: true,
            ext: false,
        };

        cb.set_trigger(&options);

        assert_eq!(cb.get_trigger(), options);
    }

    #[test]
    fn cosmicbox_hid_reset() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        cb.reset();
    }

    #[test]
    fn cosmicbox_get_count() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        cb.set_trigger(&TriggerOptions {
            top: false,
            bottom: false,
            ext: false,
        });
        cb.reset();

        assert!(cb.get_counters().is_ok());

        assert_eq!(cb.get_counters().unwrap().coinc, 0);
    }
}
