use core::panic;

use common::hw::DeviceType;
use hidapi::{HidApi, HidDevice};
use smbioslib::{SMBiosSystemInformation, table_load_from_device};

use crate::razer::{RazerCmdStatus, RazerError, RazerPacket, RazerResult};



pub const RAZER_VENDOR_ID: u16 = 0x1532;


pub struct RazerDevice {
    pub device_type: DeviceType,
    pub serial: String,
    pub device: HidDevice,
}
 

impl RazerDevice {
    pub fn scan_devices(api: &mut HidApi) -> RazerResult<Vec<Self>> {
        let mut located: Vec<Self> = Vec::new();
        for d in api.device_list() {
            #[cfg(windows)]
            if d.vendor_id() == RAZER_VENDOR_ID && d.usage() == 0x02 {
                let device = DeviceType::from_id(d.product_id());
                if let Ok(data) = d.open_device(api) {
                    located.push(Self {
                        device_type: device,
                        serial: "UNKNOWN SN".into(),
                        device: data
                    })
                }
            }
            #[cfg(unix)]
            if d.vendor_id() == RAZER_VENDOR_ID {
                let device = DeviceType::from_id(d.product_id());
                if let Ok(data) = d.open_device(api) {
                    let mut maybe = Self {
                        device_type: device,
                        serial: "UNKNOWN SN".into(),
                        device: data,
                    };
                    maybe.get_serial_number();
                    if located.iter().find(|x| x.serial == maybe.serial).is_none() {
                        located.push(maybe)
                    }
                }
            }
        }

        Ok(located)
    }

    // Attempts to get Razers serial number and sets it to the device
    fn get_serial_number(&mut self) {
        if self.device_type.is_laptop() {
            // We do it differently
            if let Ok(table) = table_load_from_device() {
                match table.find_map(|sys_info: SMBiosSystemInformation| sys_info.serial_number()) {
                    Some(uuid) => { self.serial = uuid },
                    _ => {}
                }
            }
            return;
        }

        let packet = RazerPacket::new(0x00, 0x82, &[0u8; 16]);
        if let Ok(resp) = self.write_and_read_cmd(packet){
            if let Ok(s) = String::from_utf8(Vec::from(&resp.args[0..resp.data_size as usize])) {
                println!("Serial: {}", s);
                self.serial = s;
            }
        }  else {
            println!("Error getting Serial number for {}", self.device_type.get_name());
        }
    }

    fn get_fw_version(&mut self) {
        println!("{:?}", self.write_and_read_cmd(RazerPacket::new(0x00, 0x81, &[0u8; 2])))
    }

    pub fn write_and_read_cmd(&mut self, packet: RazerPacket) -> RazerResult<RazerPacket> {
        let mut buf: [u8; 91] = [0; 91];
        buf.copy_from_slice(&packet.create_packet());

        let mut err = RazerError::ECTimeout;
        for _ in 0..3 { // Try sending packet 3 times
            if let Err(e) = self.device.send_feature_report(&buf) {
                err = e.into();
                std::thread::sleep(std::time::Duration::from_micros(400));
                continue;
            }
            std::thread::sleep(std::time::Duration::from_micros(400));
            match self.device.get_feature_report(&mut buf) {
                Ok(read_count) => {
                    if read_count != 91 {
                        continue; // Try again!
                    }
                },
                Err(e) => {
                    err = e.into();
                    continue;
                }
            }
            let new = RazerPacket::from_raw(&buf);
            if packet.is_same(&new) {
                if new.status == RazerCmdStatus::Successful {
                    return Ok(new)
                } else {
                    err = match new.status {
                        RazerCmdStatus::Busy => {
                            std::thread::sleep(std::time::Duration::from_micros(1000));
                            RazerError::ECBusy
                        },
                        RazerCmdStatus::Failure => RazerError::ECFailure,
                        RazerCmdStatus::Timeout => RazerError::ECTimeout,
                        _ => panic!("Attempted to create Razer error from OK packet!?")
                    }
                }
            } else {
                err = RazerError::InvalidResponse
            }
        }
        Err(err)
    }

    pub fn write_cmd(&mut self, packet: RazerPacket) -> RazerResult<()> {
        Ok(self.device.send_feature_report(&packet.create_packet())?)
    }
}
