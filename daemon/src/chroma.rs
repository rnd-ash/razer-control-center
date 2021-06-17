use common::effects::{self, Colour, Effect, EffectLayer};
use rusb::UsbContext;

use crate::{device::RazerDevice, razer::RazerPacket};

#[repr(u8)]
enum LedStorage {
    NoStore,
    VarStore
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Led {
    Zero = 0x00,
    ScrollWheel = 0x01,
    Battery = 0x03,
    Logo = 0x04,
    Backlight = 0x05,
    Macro = 0x07,
    Game = 0x08,
    RedProfile = 0x0C,
    GreenProfile = 0x0D,
    BlueProfile = 0x0E,
    RightSide = 0x10,
    LeftSide = 0x11,
    Charging = 0x20,
    FastCharging = 0x21,
    FullyCharged = 0x22
}

#[derive(Debug, Copy, Clone)]
pub struct LedController{ 
    led: Led
}
/*
impl LedController {
    pub fn new(led: Led) -> Self {
        Self { led }
    }

    pub fn read_led<T: UsbContext>(&self, dev: &mut RazerDevice<T>,brightness: u8) -> u8 {
        if let Ok(resp) = dev.write_and_read_cmd(RazerPacket::new(0x03, 0x83, &[LedStorage::VarStore as u8, self.led as u8, 0x00])){
            return resp.args[2]
        }
        0
    }

    pub fn set_led<T: UsbContext>(&self, dev: &mut RazerDevice<T>,brightness: u8) -> bool {
        dev.write_and_read_cmd(RazerPacket::new(0x03, 0x03, &[LedStorage::VarStore as u8, self.led as u8, brightness])).is_ok()
    }
}
*/


pub fn set_keyboard_effect<const X: usize, const Y: usize>(dev: &mut RazerDevice, layer: &EffectLayer<X, Y>) {
    // Assume effects have been executed, so we just have to build the final matrix and submit to the keyboard
    let mut buffer: Vec<u8> = Vec::with_capacity(80); //vec![0xFF, 0x00, 0x00, X as u8, 0x00, 0x00, 0x00];
    buffer.extend_from_slice(&[0xFF, 0x00, 0x00, X as u8, 0x00, 0x00, 0x00]);
    for (idx_row, row) in layer.matrix.iter().enumerate() {
        buffer[1] = idx_row as u8;
        for key in row.iter() {
            buffer.extend_from_slice(
                unsafe { ::std::slice::from_raw_parts((key as *const Colour) as *const u8, ::std::mem::size_of::<Colour>()) }
            );
        }
        // Dispatch row
        let pkt = RazerPacket::new(0x03, 0x0b, &buffer);
        buffer.truncate(7);
        dev.write_cmd(pkt);
    }

    // Now tell the keyboard to display the frame!
    let pkt = RazerPacket::new(0x03, 0x0a, &[0x05u8, 0x00u8]);
    dev.write_and_read_cmd(pkt);
}