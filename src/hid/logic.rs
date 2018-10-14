use std::thread;
use std::time::Duration;

use hidapi::HidDevice;

use hid::packet::HidPacket;
use {CosmicBox, Counter, GenericCosmicBox, TriggerOptions};

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

    fn get_count(&self, counter: Counter) -> u16 {
        match counter {
            Counter::Top => {
                self.send(HidPacket::write_8(12, !0b000 & 0b111, 0b111))
                    .unwrap();
                let lsb: u16 = self.read_8(100).unwrap()[0].into();

                self.send(HidPacket::write_8(12, !0b001 & 0b111, 0b111))
                    .unwrap();
                let msb: u16 = self.read_8(100).unwrap()[0].into();
                lsb | msb << 8
            }
            Counter::Bottom => {
                self.send(HidPacket::write_8(12, !0b010 & 0b111, 0b111))
                    .unwrap();
                let lsb: u16 = self.read_8(100).unwrap()[0].into();

                self.send(HidPacket::write_8(12, !0b011 & 0b111, 0b111))
                    .unwrap();
                let msb: u16 = self.read_8(100).unwrap()[0].into();
                lsb | msb << 8
            }
            Counter::Ext => {
                self.send(HidPacket::write_8(12, !0b100 & 0b111, 0b111))
                    .unwrap();
                let lsb: u16 = self.read_8(100).unwrap()[0].into();

                self.send(HidPacket::write_8(12, !0b101 & 0b111, 0b111))
                    .unwrap();
                let msb: u16 = self.read_8(100).unwrap()[0].into();
                lsb | msb << 8
            }
            Counter::Coinc => {
                self.send(HidPacket::write_8(12, !0b110 & 0b111, 0b111))
                    .unwrap();
                let lsb: u16 = self.read_8(100).unwrap()[0].into();

                self.send(HidPacket::write_8(12, !0b111 & 0b111, 0b111))
                    .unwrap();
                let msb: u16 = self.read_8(100).unwrap()[0].into();
                lsb | msb << 8
            }
        }
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
}
