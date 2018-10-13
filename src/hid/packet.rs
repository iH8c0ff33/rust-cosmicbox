#[derive(Debug)]
pub struct HidPacket {
    major_cmd: u8,
    minor_cmd: u8,
    data_lsb: u8,
    data_msb: u8,
    data_hid: Option<[u8; 4]>,
    data_ext: Option<[u8; 8]>,
}

impl HidPacket {
    pub fn new() -> Self {
        HidPacket {
            major_cmd: 0,
            minor_cmd: 0,
            data_lsb: 0,
            data_msb: 0,
            data_hid: None,
            data_ext: None,
        }
    }

    pub fn command(major: u8, minor: u8, data_lsb: u8, data_msb: u8) -> Self {
        HidPacket {
            major_cmd: major,
            minor_cmd: minor,
            data_lsb,
            data_msb,
            data_hid: None,
            data_ext: None,
        }
    }

    pub fn write_8(minor: u8, data_lsb: u8, data_msb: u8) -> Self {
        HidPacket {
            major_cmd: 101,
            minor_cmd: minor,
            data_lsb,
            data_msb,
            data_hid: None,
            data_ext: None,
        }
    }
}

impl<'a> From<HidPacket> for Vec<u8> {
    fn from(packet: HidPacket) -> Vec<u8> {
        let mut raw = vec![
            packet.major_cmd,
            packet.minor_cmd,
            packet.data_lsb,
            packet.data_msb,
        ];

        if let Some(data) = packet.data_hid {
            raw.extend_from_slice(&data)
        };

        if let Some(data) = packet.data_ext {
            raw.extend_from_slice(&data);
        };

        raw
    }
}
