extern crate nearfield_sys;

use std::fmt;
use std::fmt::Display;

pub type NFCResult<T> = Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
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
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Chip => write!(f, "Device internal chip error"),
            Error::NotSupported => write!(f, "Operation not supported by device"),
            Error::InvalidArgument => write!(f, "Invalid argument(s)"),
            Error::InputOutput => write!(
                f,
                "Input/output error - device may not be usable anymore without re-opening it"
            ),
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
            _ => write!(f, "Unknown error"),
        }
    }
}
