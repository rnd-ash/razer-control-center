use std::{cmp::min, fs::read};


pub type RazerResult<T> = std::result::Result<T, RazerError>;


#[derive(Debug)]
pub enum RazerError {
    UsbError(rusb::Error),
    HidError(hidapi::HidError),
    CmdNotSupported,
    ECBusy,
    ECTimeout,
    InvalidResponse,
    ECFailure
}

impl From<rusb::Error> for RazerError {
    fn from(x: rusb::Error) -> Self {
        Self::UsbError(x)
    }
}

impl From<hidapi::HidError> for RazerError {
    fn from(x: hidapi::HidError) -> Self {
        Self::HidError(x)
    }
}


#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RazerCmdStatus {
    New = 0,
    Busy = 1,
    Successful = 2,
    Failure = 3,
    Timeout = 4,
    NotSupported = 5
}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RazerPacket {
    report_index: u8,
    pub status: RazerCmdStatus,
    pub id: u8,
    pub remaining: u16,
    pub protocol_type: u8,
    pub data_size: u8,
    pub cmd_class: u8,
    pub cmd_id: u8,
    pub args: [u8; 80],
    pub crc: u8,
    pub _res: u8
}

impl std::fmt::Display for RazerPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Razer packet {{ cmd: {:02X}, class: {:02X}, args: {:02X?}, status: {:?}}}", self.cmd_id, self.cmd_class, &self.args[0..self.data_size as usize], self.status))
    }
}

impl RazerPacket {

    pub fn set_crc(&mut self) {
        let raw = unsafe { ::std::slice::from_raw_parts((self as *const  Self) as *const u8, ::std::mem::size_of::<Self>()) };
        self.crc = 0;
        raw[2..88].iter().for_each(|x| self.crc ^= *x);
    }

    pub fn new(class: u8, cmd: u8, args: &[u8]) -> Self {
        let max = min(80, args.len());
        let mut tmp = Self {
            report_index: 0,
            status: RazerCmdStatus::New,
            id: 0xFF,
            remaining: 0x00,
            protocol_type: 0x00,
            data_size: max as u8,
            cmd_class: class,
            cmd_id: cmd,
            args: [0; 80],
            crc: 0,
            _res: 0x00
        };
        tmp.args[0..max].copy_from_slice(&args[0..max]);
        tmp.set_crc();
        tmp
    }

    pub const fn is_same(&self, other: &Self) -> bool {
        self.remaining == other.remaining && self.cmd_class == other.cmd_class && self.id == other.id
    }

    pub fn set_args(&mut self, args: &[u8]) {
        let max = min(80, args.len());
        self.args[0..max].copy_from_slice(&args[0..max]);
        self.data_size = max as u8;
        self.set_crc();
    }

    pub fn create_packet(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts((self as *const Self) as *const u8, ::std::mem::size_of::<Self>()) }
    }

    pub fn from_raw(buf: &[u8; 91]) -> Self {
        unsafe { std::ptr::read(buf.as_ptr() as *const Self) }
    }
}