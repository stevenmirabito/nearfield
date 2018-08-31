extern crate nearfield_sys;

use std::fmt;

pub type NFCResult = Result<(), Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    Chip,
    NotSupported,
    InvalidArgument,
    InputOutput,
    MifareAuthFailure,
    NotImplemented,
    NoSuchDevice,
    OpenDevice,
    OperationAborted,
    BufferOverflow,
    Transmission,
    Software,
    TargetReleased,
    TimedOut,
    Success,
    Unknown,
}

impl Error {
    pub(crate) fn from_sys(sys_error: i32) -> Error {
        if sys_error == 0 {
            return Error::Success;
        }

        match sys_error {
            nearfield_sys::NFC_ECHIP => Error::Chip,
            nearfield_sys::NFC_EDEVNOTSUPP => Error::NotSupported,
            nearfield_sys::NFC_EINVARG => Error::InvalidArgument,
            nearfield_sys::NFC_EIO => Error::InputOutput,
            nearfield_sys::NFC_EMFCAUTHFAIL => Error::MifareAuthFailure,
            nearfield_sys::NFC_ENOTIMPL => Error::NotImplemented,
            nearfield_sys::NFC_ENOTSUCHDEV => Error::NoSuchDevice,
            nearfield_sys::NFC_EOPABORTED => Error::OperationAborted,
            nearfield_sys::NFC_EOVFLOW => Error::BufferOverflow,
            nearfield_sys::NFC_ERFTRANS => Error::Transmission,
            nearfield_sys::NFC_ESOFT => Error::Software,
            nearfield_sys::NFC_ETGRELEASED => Error::TargetReleased,
            nearfield_sys::NFC_ETIMEOUT => Error::TimedOut,
            _ => Error::Unknown,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Chip => write!(f, "Device internal chip error"),
            Error::NotSupported => write!(f, "Operation not supported by device"),
            Error::InvalidArgument => write!(f, "Invalid argument(s)"),
            Error::InputOutput => write!(f, "Input/output error - device may not be usable anymore without re-opening it"),
            Error::MifareAuthFailure => write!(f, "MIFARE authentication failed"),
            Error::NotImplemented => write!(f, "Not (yet) implemented"),
            Error::NoSuchDevice => write!(f, "No such device"),
            Error::OpenDevice => write!(f, "Unable to open device (invalid connection string?)"),
            Error::OperationAborted => write!(f, "Operation aborted (by user)"),
            Error::BufferOverflow => write!(f, "Buffer overflow"),
            Error::Transmission => write!(f, "Error during RF transmission"),
            Error::Software => write!(f, "Software error (allocation, file/pipe creation, etc.)"),
            Error::TargetReleased => write!(f, "Target released"),
            Error::TimedOut => write!(f, "Operation timed out"),
            Error::Success => write!(f, "Success (no error)"),
            _ => write!(f, "Unknown error"),
        }
    }
}