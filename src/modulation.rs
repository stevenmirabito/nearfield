extern crate nearfield_sys;

use std::fmt;
use baud_rate::BaudRate;
use modulation_type::ModulationType;

pub(crate) type InternalModulation = nearfield_sys::nfc_modulation;

pub struct Modulation {
    pub modulation_type: ModulationType,
    pub baud_rate: BaudRate,
}

impl Modulation {
    pub(crate) fn from_sys(internal: InternalModulation) -> Modulation {
        Modulation {
            modulation_type: ModulationType::from_sys(internal.nmt),
            baud_rate: BaudRate::from_sys(internal.nbr),
        }
    }

    pub(crate) fn to_sys(&self) -> InternalModulation {
        InternalModulation {
            nmt: self.modulation_type.to_sys(),
            nbr: self.baud_rate.to_sys(),
        }
    }
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.modulation_type, self.baud_rate)
    }
}

pub static COMMON: &'static [Modulation] = &[
    Modulation { modulation_type: ModulationType::ISO14443A, baud_rate: BaudRate::K106 },
    Modulation { modulation_type: ModulationType::ISO14443B, baud_rate: BaudRate::K106 },
    Modulation { modulation_type: ModulationType::FELICA, baud_rate: BaudRate::K212 },
    Modulation { modulation_type: ModulationType::FELICA, baud_rate: BaudRate::K424 },
    Modulation { modulation_type: ModulationType::JEWEL, baud_rate: BaudRate::K106 },
];

pub static MIFARE: &'static [Modulation] = &[
    Modulation { modulation_type: ModulationType::ISO14443A, baud_rate: BaudRate::K106 },
];

pub static FELICA: &'static [Modulation] = &[
    Modulation { modulation_type: ModulationType::FELICA, baud_rate: BaudRate::K212 },
    Modulation { modulation_type: ModulationType::FELICA, baud_rate: BaudRate::K424 },
];

pub static JEWEL: &'static [Modulation] = &[
    Modulation { modulation_type: ModulationType::JEWEL, baud_rate: BaudRate::K106 },
];
