extern crate nearfield_sys;

use baud_rate::BaudRate;
use modulation_type::ModulationType;
use std::fmt;

pub(crate) type InternalModulation = nearfield_sys::nfc_modulation;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Modulation {
    pub modulation_type: ModulationType,
    pub baud_rate: BaudRate,
}

impl From<InternalModulation> for Modulation {
    fn from(internal: InternalModulation) -> Self {
        Modulation {
            modulation_type: ModulationType::from(internal.nmt),
            baud_rate: BaudRate::from(internal.nbr),
        }
    }
}

impl Into<InternalModulation> for Modulation {
    fn into(self) -> InternalModulation {
        InternalModulation {
            nmt: self.modulation_type.into(),
            nbr: self.baud_rate.into(),
        }
    }
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.modulation_type, self.baud_rate)
    }
}

pub static COMMON: &'static [Modulation] = &[
    Modulation {
        modulation_type: ModulationType::ISO14443A,
        baud_rate: BaudRate::K106,
    },
    Modulation {
        modulation_type: ModulationType::ISO14443B,
        baud_rate: BaudRate::K106,
    },
    Modulation {
        modulation_type: ModulationType::FELICA,
        baud_rate: BaudRate::K212,
    },
    Modulation {
        modulation_type: ModulationType::FELICA,
        baud_rate: BaudRate::K424,
    },
    Modulation {
        modulation_type: ModulationType::JEWEL,
        baud_rate: BaudRate::K106,
    },
];

pub static MIFARE: &'static [Modulation] = &[Modulation {
    modulation_type: ModulationType::ISO14443A,
    baud_rate: BaudRate::K106,
}];

pub static FELICA: &'static [Modulation] = &[
    Modulation {
        modulation_type: ModulationType::FELICA,
        baud_rate: BaudRate::K212,
    },
    Modulation {
        modulation_type: ModulationType::FELICA,
        baud_rate: BaudRate::K424,
    },
];

pub static JEWEL: &'static [Modulation] = &[Modulation {
    modulation_type: ModulationType::JEWEL,
    baud_rate: BaudRate::K106,
}];
