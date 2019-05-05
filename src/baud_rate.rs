extern crate nearfield_sys;

use std::fmt;
use modulation_type::InternalModulationType;

pub(crate) type InternalBaudRate = nearfield_sys::nfc_baud_rate;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BaudRate {
    K106,
    K212,
    K424,
    K847,
    UNDEFINED,
}

impl From<InternalBaudRate> for BaudRate {
    fn from(internal: InternalBaudRate) -> Self {
        match internal {
            nearfield_sys::nfc_baud_rate::NBR_106 => BaudRate::K106,
            nearfield_sys::nfc_baud_rate::NBR_212 => BaudRate::K212,
            nearfield_sys::nfc_baud_rate::NBR_424 => BaudRate::K424,
            nearfield_sys::nfc_baud_rate::NBR_847 => BaudRate::K847,
            _ => BaudRate::UNDEFINED,
        }
    }
}

impl Into<InternalBaudRate> for BaudRate {
    fn into(self) -> InternalBaudRate {
        match self {
            BaudRate::K106 => nearfield_sys::nfc_baud_rate::NBR_106,
            BaudRate::K212 => nearfield_sys::nfc_baud_rate::NBR_212,
            BaudRate::K424 => nearfield_sys::nfc_baud_rate::NBR_424,
            BaudRate::K847 => nearfield_sys::nfc_baud_rate::NBR_847,
            _ => nearfield_sys::nfc_baud_rate::NBR_UNDEFINED,
        }
    }
}

impl fmt::Display for BaudRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BaudRate::K106 => write!(f, "106 kbps"),
            BaudRate::K212 => write!(f, "212 kbps"),
            BaudRate::K424 => write!(f, "424 kbps"),
            BaudRate::K847 => write!(f, "847 kbps"),
            _ => write!(f, "Unknown Baud Rate"),
        }
    }
}
