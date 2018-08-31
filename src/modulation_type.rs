extern crate nearfield_sys;

use std::fmt;

pub type InternalModulationType = nearfield_sys::nfc_modulation_type;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModulationType {
    ISO14443A,
    ISO14443B,
    ISO14443BI,
    ISO14443B2SR,
    ISO14443B2CT,
    JEWEL,
    FELICA,
    DEP,
}

impl ModulationType {
    pub(crate) fn from_sys(internal: InternalModulationType) -> ModulationType {
        match internal {
            nearfield_sys::nfc_modulation_type::NMT_ISO14443A => ModulationType::ISO14443A,
            nearfield_sys::nfc_modulation_type::NMT_ISO14443B => ModulationType::ISO14443B,
            nearfield_sys::nfc_modulation_type::NMT_ISO14443BI => ModulationType::ISO14443BI,
            nearfield_sys::nfc_modulation_type::NMT_ISO14443B2CT => ModulationType::ISO14443B2CT,
            nearfield_sys::nfc_modulation_type::NMT_ISO14443B2SR => ModulationType::ISO14443B2SR,
            nearfield_sys::nfc_modulation_type::NMT_FELICA => ModulationType::FELICA,
            nearfield_sys::nfc_modulation_type::NMT_JEWEL => ModulationType::JEWEL,
            nearfield_sys::nfc_modulation_type::NMT_DEP => ModulationType::DEP,
        }
    }

    pub(crate) fn to_sys(self) -> InternalModulationType {
        match self {
            ModulationType::ISO14443A => nearfield_sys::nfc_modulation_type::NMT_ISO14443A,
            ModulationType::ISO14443B => nearfield_sys::nfc_modulation_type::NMT_ISO14443B,
            ModulationType::ISO14443BI => nearfield_sys::nfc_modulation_type::NMT_ISO14443BI,
            ModulationType::ISO14443B2CT => nearfield_sys::nfc_modulation_type::NMT_ISO14443B2CT,
            ModulationType::ISO14443B2SR => nearfield_sys::nfc_modulation_type::NMT_ISO14443B2SR,
            ModulationType::FELICA => nearfield_sys::nfc_modulation_type::NMT_FELICA,
            ModulationType::JEWEL => nearfield_sys::nfc_modulation_type::NMT_JEWEL,
            ModulationType::DEP => nearfield_sys::nfc_modulation_type::NMT_DEP,
        }
    }
}

impl fmt::Display for ModulationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ModulationType::ISO14443A => write!(f, "ISO/IEC 14443A"),
            &ModulationType::ISO14443B => write!(f, "ISO/IEC 14443-4B"),
            &ModulationType::ISO14443BI => write!(f, "ISO/IEC 14443-4B'"),
            &ModulationType::ISO14443B2CT => write!(f, "ISO/IEC 14443-2B ASK CTx"),
            &ModulationType::ISO14443B2SR => write!(f, "ISO/IEC 14443-2B ST SRx"),
            &ModulationType::FELICA => write!(f, "FeliCa"),
            &ModulationType::JEWEL => write!(f, "Innovision Jewel"),
            &ModulationType::DEP => write!(f, "ISO-DEP"),
        }
    }
}