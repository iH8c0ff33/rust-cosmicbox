use hidapi::{HidDevice, HidResult};

use hid::packet::HidPacket;
use CosmicBox;

impl CosmicBox<HidDevice> {
    pub fn send(&self, packet: HidPacket) -> HidResult<()> {
        self.device.send_feature_report(&<Vec<u8>>::from(packet))
    }

    pub fn read_8(&self, report: u8) -> HidResult<Vec<u8>> {
        let mut data = [report; 8];
        self.device.get_feature_report(&mut data)?;
        Ok(data.to_vec())
    }

    pub fn read_16(&self, report: u8) -> HidResult<Vec<u8>> {
        let mut data = [report; 16];
        self.device.get_feature_report(&mut data)?;
        Ok(data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use hidapi::HidApi;

    use hid::packet::HidPacket;
    use CosmicBox;

    #[test]
    fn cosmicbox_hid_send() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        assert!(
            cb.send(HidPacket::write_8(12, !0b000 & 0b111, 0b000))
                .is_ok()
        );
    }

    #[test]
    fn cosmicbox_hid_read8() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        assert!(cb.read_8(100).is_ok());
    }

    #[test]
    fn cosmicbox_hid_read16() {
        let hid = HidApi::new().unwrap();
        let cb = CosmicBox::connect(hid);

        assert!(cb.read_8(104).is_ok());
    }
}
