
// Keyboards
const KEYBOARD_IDS: &[(u16, &'static str)] = &[
    (0x0235, "Blackwidow Lite (2018)")
];

// Laptops
const LAPTOP_IDS: &[(u16, &'static str)] = &[
    // 15"
    (0x0224, "Razer blade 15 2016"),
    (0x0233, "Razer Blade 15 2018 (Adv)"),
    (0x023A, "Razer blade 15 2019 (Adv)"),
    (0x023B, "Razer blade 15 2018 (Base)"),
    (0x0240, "Razer blade 15 2018 (Mercury edition)"),
    (0x0245, "Razer blade 15 mid 2019 (Mercury edition)"),
    (0x024B, "Razer blade 15 late 2019 (Adv)"),
    (0x024D, "Razer blade 15 2019 (Studio edition)"),
    (0x0253, "Razer blade 15 2020 (Adv)"),
    (0x0255, "Razer blade 15 2020 (Base)"),

    // Stealth
    (0x022D, "Razer blade Stealth 2017 (Mid)"),
    (0x0232, "Razer blade Stealth 2017 (End)"),
    (0x0239, "Razer blade Stealth 2019"),
    (0x024A, "Razer blade Stealth 2019 (GTX)"),
    (0x0252, "Razer blade Stealth 2020"),

    // Pro 17"
    (0x0116, "Razer blade Pro 2015"),
    (0x0210, "Razer blade Pro 2016"),
    (0x0225, "Razer blade Pro 2017"),
    (0x022F, "Razer blade Pro 2018 (FHD)"),
    (0x0234, "Razer blade Pro 2019"),
    (0x024C, "Razer blade Pro late 2019"),
    (0x0256, "Razer blade Pro 2020 (FHD)"),

    // Other
    (0x020F, "Razer blade QHD"),
];

// Mice
const MICE_IDS: &[(u16, &'static str)] = &[

];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceType {
    Laptop(u16, &'static str),
    Keyboard(u16, &'static str),
    Mouse(u16, &'static str),
    Unknown(u16)
}

impl DeviceType {
    pub fn from_id(id: u16) -> Self {
        if let Some(kbd) = KEYBOARD_IDS.iter().find(|x| x.0 == id) {
            return Self::Keyboard(kbd.0, kbd.1)
        }

        if let Some(laptop) = LAPTOP_IDS.iter().find(|x| x.0 == id) {
            return Self::Laptop(laptop.0, laptop.1)
        }

        if let Some(mouse) = MICE_IDS.iter().find(|x| x.0 == id) {
            return Self::Mouse(mouse.0, mouse.1)
        }
        DeviceType::Unknown(id)
    }

    pub fn is_laptop(&self) -> bool {
        if let Self::Laptop(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_keyboard(&self) -> bool {
        if let Self::Keyboard(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_mouse(&self) -> bool {
        if let Self::Mouse(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            DeviceType::Laptop(_, s) => s,
            DeviceType::Keyboard(_, s) => s,
            DeviceType::Mouse(_, s) => s,
            DeviceType::Unknown(_) => "UNKNOWN",
        }
    }
} 