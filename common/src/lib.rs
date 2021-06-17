pub mod keyboard;
pub mod effects;
pub mod hw;

/// Struct's in this library get passed between  the daemon and CLI/GUI


struct RazerLaptop {
    has_logo_control: bool,
    fan_zone_count: u8,
    min_fan_rpm: u32,
    max_fan_rpm: u32,
    has_gaming_mode: bool,
    has_creator_mode: bool,
    keyboard: RazerKeyboard
}

struct RazerKeyboard {
    matrix_type: RGBControl,

}

enum RGBControl {
    // Device has 1 colour, and 1 zone
    OneColourOneZone,
    // Device has  multiple colours but only 1 zone
    MultiColourOneZone,
    // Device  has multiple colours and multiple zones  (Per key RGB)
    MultiColourMultiZone,
}

struct RazerCommonDevice {

}