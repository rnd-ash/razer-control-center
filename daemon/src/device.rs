use rusb::{Device, DeviceDescriptor, DeviceHandle, Direction, EndpointDescriptor, EndpointDescriptors, GlobalContext, Result, TransferType, UsbContext};

use crate::{TIMEOUT, razer::{self, RazerCmdStatus, RazerError, RazerPacket, RazerResult}};

const RAZER_VENDOR_ID: u16 = 0x1532;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum DeviceType {
    /// Laptop - Power and keyboard backlight controls are available
    Laptop(&'static str),
    /// Mouse
    Mouse(&'static str),
    /// Keyboard - Only RGB control is available
    Keyboard(&'static str),
    /// Unknown device. Will show up as 'generic'
    Unknown(u16)
}


impl DeviceType {
    pub fn from_id(id: u16) -> Self {
        match id {
            0x023B => Self::Laptop("Razer blade 2018 (Base)"),
            0x0233 => Self::Laptop("Razer blade 2018 (Adv)"),
            0x0244 => Self::Laptop("Razer blade 2019 (Adv)"),
            x => Self::Unknown(x)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Endpoint {
    pub (crate) cfg: u8,
    pub (crate) iface: u8,
    pub (crate) setting: u8,
    pub (crate) addr: u8,
    pub (crate) has_kernel_driver: bool,
    pub (crate) transfer_type: TransferType
}

pub struct RazerDevice<T: UsbContext> {
    pub device_type: DeviceType,
    handle: DeviceHandle<T>,
    device_descriptor: DeviceDescriptor,
    device: Device<T>,
    endpoint: Endpoint
}

impl<T: UsbContext> RazerDevice<T> {
    pub fn new(ctx: &mut T, pid: u16) -> Result<Self> {
        let dev_type = DeviceType::from_id(pid);

        let dev_list = ctx.devices()?;
        for dev in dev_list.iter() {
            let desc = dev.device_descriptor()?;

            if desc.vendor_id() == RAZER_VENDOR_ID && desc.product_id() == pid {
                let mut handle  = dev.open()?;

                for n in 0..desc.num_configurations() {
                    let cfg = match dev.config_descriptor(n) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Error reading cfg {}", e);
                            continue;
                        }
                    };
                    
                    let mut endpoints: Vec<Endpoint> = Vec::new();

                    for iface in cfg.interfaces() {
                        for iface_desc in iface.descriptors() {
                            for endpoint_desc in iface_desc.endpoint_descriptors() {
                                if endpoint_desc.direction() == Direction::In 
                                && iface_desc.protocol_code() == 0x02 { // Mouse
                                    let has_kernel_driver = handle.kernel_driver_active(iface_desc.interface_number()).unwrap_or(false);

                                    endpoints.push(Endpoint{
                                        cfg: cfg.number(),
                                        iface: iface_desc.interface_number(),
                                        setting: iface_desc.setting_number(),
                                        addr: iface_desc.setting_number(),
                                        transfer_type: endpoint_desc.transfer_type(),
                                        has_kernel_driver
                                    })
                                }
                            }
                        }
                    }

                    // On razer laptops, multiple endpoints are located for the laptops EC
                    // First tends to be keyboard
                    // Second tends to be the mouse
                    // Only the last one is the EC, and so we can detach the kernel driver PERMANENTLY
                    // This saves us spamming kernel log, and also reduces some overhead :)

                    if endpoints.len() > 1 {
                        let laptop_maybe_endpoint = endpoints.last().unwrap();
                        if laptop_maybe_endpoint.has_kernel_driver {
                            handle.detach_kernel_driver(laptop_maybe_endpoint.iface)?;
                        }
                        // Now we don't have a kernel driver! No more spamming the kernel
                        endpoints.last_mut().unwrap().has_kernel_driver = false;
                    }

                    let endpoint = endpoints.last().unwrap();
                    return Ok(Self {
                        device_type: dev_type,
                        handle,
                        device_descriptor: desc,
                        device: dev,
                        endpoint: endpoint.clone()
                    })
                }
            }
        }
        Err(rusb::Error::NoDevice)
    }

    pub fn write_and_read_cmd(&mut self, packet: RazerPacket) -> RazerResult<RazerPacket> {
        if self.endpoint.has_kernel_driver {
            self.handle.detach_kernel_driver(self.endpoint.iface)?
        }

        #[cfg(windows)]
        {
            self.handle.set_active_configuration(self.endpoint.cfg)?;
            self.handle.claim_interface(self.endpoint.iface)?;
            self.handle.set_alternate_setting(self.endpoint.iface, self.endpoint.setting)?
        }

        let mut buf = packet.create_packet();

        let mut last_err : Option<RazerError> = None;

        for _ in 0..3 { // Try 3 times
            if let Err(e) = self.handle.write_control(0x21, 0x09, 0x300, self.endpoint.iface as u16, &buf, TIMEOUT) {
                last_err = Some(e.into());
                continue
            }
            std::thread::sleep(std::time::Duration::from_micros(500));

            for i in &mut buf { *i = 0 }; // Compiler turns this into memset!
            if let Err(e) = self.handle.read_control(0xA1, 0x01, 0x300, self.endpoint.iface as u16, &mut buf, TIMEOUT) {
                last_err = Some(e.into());
                continue
            }

            let pkt = RazerPacket::from_raw(&buf);

            if packet.is_same(&pkt) {
                if pkt.status == RazerCmdStatus::Successful {
                    if  self.endpoint.has_kernel_driver {
                        self.handle.attach_kernel_driver(self.endpoint.iface).ok();
                    }
                    return Ok(pkt)
                } else {
                    last_err = Some(match pkt.status {
                        RazerCmdStatus::Busy => RazerError::ECBusy,
                        RazerCmdStatus::Failure => RazerError::ECFailure,
                        RazerCmdStatus::Timeout => RazerError::ECTimeout,
                        RazerCmdStatus::NotSupported => RazerError::CmdNotSupported,
                        _ => RazerError::InvalidResponse // This should never happen, but cover incase
                    })
                }
            } else {
                last_err = Some(RazerError::InvalidResponse)
            }
        }

        if  self.endpoint.has_kernel_driver {
            self.handle.attach_kernel_driver(self.endpoint.iface).ok();
        }
        Err(last_err.unwrap())
    }
}
