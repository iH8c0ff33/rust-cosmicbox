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
