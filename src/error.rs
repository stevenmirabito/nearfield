extern crate nearfield_sys;

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
    fn as_str(&self) -> &'static str {
        match *self {
            Error::Chip => "Device internal chip error",
            Error::NotSupported => "Operation not supported by device",
            Error::InvalidArgument => "Invalid argument(s)",
            Error::InputOutput => "Input/output error - device may not be usable anymore without re-opening it",
            Error::MifareAuthFailure => "MIFARE authentication failed",
            Error::NotImplemented => "Not (yet) implemented",
            Error::NoSuchDevice => "No such device",
            Error::OpenDevice => "Unable to open device (invalid connection string?)",
            Error::OperationAborted => "Operation aborted (by user)",
            Error::BufferOverflow => "Buffer overflow",
            Error::Transmission => "Error during RF transmission",
            Error::Software => "Software error (allocation, file/pipe creation, etc.)",
            Error::TargetReleased => "Target released",
            Error::TimedOut => "Operation timed out",
            Error::Success => "Success (no error)",
            _ => "Unknown error",
        }
    }

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